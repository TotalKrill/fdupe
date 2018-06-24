#![feature(test)]
extern crate rayon;
extern crate sha1;

use std::path;
use std::fs;

mod file;
mod hasher;
mod lazyfile;

pub use file::FileContent;
pub use std::fs::Metadata;
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

    #[test]
    fn identify_img_file() {
        let oristr =  &String::from("./testdata/cats.jpg");
        let original = FileContent::from_path( &path::PathBuf::from( oristr ))
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
        let original = FileContent::from_path( &path::PathBuf::from( oristr ))
            .unwrap();

        let comparefiles: Vec< Result<FileContent, std::io::Error>> = files.par_iter()
        .map( |x| FileContent::from_path( &x ) )
        .collect();

        let mut dupes: i64 = 0;
        for f in comparefiles {
            if f.unwrap() == original {
                dupes += 1;
            }
        }
        println!("dupes: {}", dupes);
        assert_eq!(dupes, 3);


    }

    #[test]
    fn find_no_dupes() {

        let oristr =  &String::from("./testdata/a");
        let testdir = &String::from( "./testdata" );
        let files = get_files_recursive_at( testdir )
            .expect("Failed getting the files");
        let original = FileContent::from_path( &path::PathBuf::from( oristr ))
            .expect("Error with original");


    }

    #[bench]
    fn bench_new_duplicatereports(b: &mut Bencher) {
        b.iter(|| {
        });
    }

}
