#![feature(test)]

extern crate md5;

use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::fmt;
use std::path;
use std::fs;


#[derive(PartialEq, Eq, Clone)]
pub struct FileIdentification {
    pub path: String,
    pub hash: md5::Digest,
    pub size: u64,
}

impl FileIdentification {
    pub fn new( path: &path::Path ) -> Result<FileIdentification, std::io::Error > {

        let file = fs::canonicalize( path )?;
        let hash: md5::Digest  = get_hash_at( path )?;
        let size = fs::metadata(path)?.len();

        Ok( FileIdentification{
                path: file.display().to_string() ,
                hash,
                size,
        } )

    }

    pub fn is_duplicate_of(&self, other: &FileIdentification) -> bool {
        let duplicate: bool = ( self.path != other.path ) &&
            ( self.hash == other.hash ) &&
            ( self.size == other.size );
        duplicate
    }

}

impl fmt::Display for FileIdentification
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:x}", self.path, self.hash)
    }
}

pub struct DuplicateReport {
    original: FileIdentification,
    duplicates: Vec< FileIdentification >,
}

impl DuplicateReport {

    pub fn new( original: &FileIdentification, check_files: &Vec< FileIdentification > ) -> DuplicateReport {

        let mut duplicates: Vec< FileIdentification> = Vec::new();
        for file in check_files {
            if file.is_duplicate_of( original ) {
                duplicates.push( file.clone() );
            }
        }

        let report: DuplicateReport = DuplicateReport{ original: original.clone(), duplicates };
        report
    }

    pub fn number_duplicates(&self) -> usize {
        self.duplicates.len()
    }


    pub fn size_duplicates(&self) -> u64 {
        let sum: u64 = self.original.size * self.duplicates.len() as u64;
        sum
    }

    pub fn print(&self) {
        println!("{}", self.original);
        println!("Has {} duplicates, totaling {} Bytes",
                self.duplicates.len(),
                self.size_duplicates() );
        println!("==========================================");
        for dupe in &self.duplicates {
            println!("{}", dupe);
        }
    }
}

pub fn get_hash_at( _path : &path::Path ) -> Result< md5::Digest, std::io::Error>
{
    let f = File::open( _path )?;

    let mut s = String::new();

    BufReader::new( f ).read_to_string(&mut s)?;

    let bytes =  s.into_bytes();

    let hash = md5::compute( bytes );

    Ok( hash )
}

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

pub fn hash_vector( vec: Vec< path::PathBuf > )
    -> Vec< Result< FileIdentification, std::io::Error > >{

    let hashed: Vec< Result<FileIdentification, std::io::Error>> = vec.iter()
        .map( |x| FileIdentification::new( &x ) )
        .collect();
    hashed
}

extern crate test;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_files_ignore_errors() {
        let files = get_files_recursive_at( &String::from( "./testdata" ) )
            .expect("Failed getting the files");

        assert_eq!( files.len(), 6, "got wrong amount of files");
    }

    #[test]
    fn find_dupes_of_original() {

        let oristr =  &String::from("./testdata/original");
        let testdir = &String::from( "./testdata" );
        let files = get_files_recursive_at( testdir )
            .expect("Failed getting the files");
        let original = FileIdentification::new( &path::PathBuf::from( oristr ))
            .expect("Error with original");

        let files: Vec<FileIdentification> = hash_vector( files ).into_iter()
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
        let original = FileIdentification::new( &path::PathBuf::from( oristr ))
            .expect("Error with original");

        let files: Vec<FileIdentification> = hash_vector( files ).into_iter()
            .filter_map( |res| res.ok() )
            .collect();

        let report: DuplicateReport = DuplicateReport::new( &original, &files );

        assert_eq!( report.number_duplicates(), 0 );
    }


}
