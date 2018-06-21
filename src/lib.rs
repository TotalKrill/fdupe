#![feature(test)]
extern crate rayon;
extern crate sha1;

use rayon::prelude::*;

use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::fmt;
use std::path;
use std::fs;

mod file;
mod hasher;
mod lazyfile;
mod metadata;

pub use file::FileContent;
pub use metadata::Metadata;
pub use hasher::Hasher;

pub fn get_files_recursive_at( path: &str ) -> Result< Vec<path::PathBuf>, std::io::Error> {
    let path = path::PathBuf::from( path );
    Ok( get_files_recursive( &path ))
}

/// Uses the given path and recursively visits all subfolder, return all files and ignores any errors
pub fn get_files_recursive( _path: &path::Path ) -> Vec<path::PathBuf>
{
    let mut files: Vec<path::PathBuf> = Vec::new();
    // early exit if it is just one file
    if !_path.is_dir() {
        let file = fs::canonicalize( _path );
        match file {
            Ok(v) => { files.push( v ); },
            Err(_e) => { println!("{:?}", _e ) },
        }
        return files;
    }

    let paths = fs::read_dir( _path ).unwrap();

    for path in paths {
        match path {
            Ok(d) => {
                if !d.path().is_dir() {
                    let file = fs::canonicalize( d.path() );
                    match file {
                        Ok(v) => { files.push( v ); },
                        Err(_e) => { continue; },
                        }

                }
                else {
                    let mut dirfiles = get_files_recursive( &d.path() );
                    files.append( &mut dirfiles );
                }
            },
            Err(e) => {
                println!("error: {:?}", e );
            }
        }
    }
    files
}

extern crate test;

#[cfg(test)]

mod tests {
    use super::*;
    use test::Bencher;
    #[bench]
    fn get_hash_md5(b: &mut Bencher) {
        let oristr =  &String::from("./testdata/cats.jpg");
        let dig = get_md5_hash_at( &path::PathBuf::from(oristr) )
        .expect("Could not get hash");

        let hash: [u8; 16] =
            [0xc4,0x7f,0xac,0x96,0xd7,0x35,0xc6,0xc5,0x88,0xab,0xac,0x8d,0xf0,0x38,0x2f,0xdc];

        assert!( &dig == &hash, "Dig was {:?}", dig);

        b.iter(|| get_md5_hash_at( &path::PathBuf::from(oristr) ) )
    }

    #[test]
    fn identify_img_file() {
        let oristr =  &String::from("./testdata/cats.jpg");
        let original = FileContent::new( &path::PathBuf::from( oristr ))
            .expect("Error with original");
    }

    #[test]
    fn find_files_ignore_errors() {
        let files = get_files_recursive_at( &String::from( "./testdata" ) )
            .expect("Failed getting the files");

        assert_eq!( files.len(), 22, "got wrong amount of files");
    }

    #[test]
    fn find_dupes_of_original() {

        let oristr =  &String::from("./testdata/original");
        let testdir = &String::from( "./testdata" );
        let files = get_files_recursive_at( testdir )
            .expect("Failed getting the files");
        let original = FileContent::new( &path::PathBuf::from( oristr ))
            .expect("Error with original");

        let files: Vec<FileContent> = hash_vector( &files ).into_iter()
            .filter_map( |res| res.ok() )
            .collect();

        let report: DuplicateReport = DuplicateReport::new( &original, &files );

        assert_eq!( report.number_duplicates(), 3 );

    }

    #[test]
    fn find_no_dupes() {

        let oristr =  &String::from("./testdata/a");
        let testdir = &String::from( "./testdata" );
        let files = get_files_recursive_at( testdir )
            .expect("Failed getting the files");
        let original = FileContent::new( &path::PathBuf::from( oristr ))
            .expect("Error with original");

        let files: Vec<FileContent> = hash_vector( &files ).into_iter()
            .filter_map( |res| res.ok() )
            .collect();

        let report: DuplicateReport = DuplicateReport::new( &original, &files );

        assert_eq!( report.number_duplicates(), 0 );

    }

    #[bench]
    fn bench_new_duplicatereports(b: &mut Bencher) {
        b.iter(|| {
            let oristr =  &String::from("./testdata/cats.jpg");
            let testdir = &String::from( "./testdata" );
            let files = get_files_recursive_at( testdir )
                .expect("Failed getting the files");
            let original = FileContent::new( &path::PathBuf::from( oristr ))
                .expect("Error with original");

            let files: Vec<FileContent> = hash_vector( &files ).into_iter()
                .filter_map( |res| res.ok() )
                .collect();
            let rep = DuplicateReport::new( &original, &files );
            assert_eq!(rep.duplicates().len(), 2, "Did not find enought cat dupes");
        });

    }

}
