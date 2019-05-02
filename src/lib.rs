#![feature(drain_filter)]
#![feature(test)]

extern crate sha1;
extern crate walkdir;


pub mod file;
mod hasher;
mod lazyfile;

pub use crate::file::FileContent;

use std::io;
use std::io::Write;
use walkdir::WalkDir;

pub struct DupeSet<'a> {
    pub original: &'a FileContent,
    pub dupes: Vec<FileContent>,
}

/// Gets only the files from the specified directory with specified traversal depth of subfolders
/// # Examples
/// ```
///    use filedupes::get_files;
///    // get the files in the closest subfolder
///    let files = get_files("./",2);
/// ```
/// ```
///    use filedupes::get_files;
///    // traverse all the subfolder
///    let all_files = get_files("./",std::usize::MAX);
/// ```
/// ```
///    use filedupes::get_files;
///    // get only one file as filecontent, to be lazily compared
///    let only_one_file = get_files("./file",0);
/// ```
pub fn get_files(path: &str, depth: usize) -> Vec<FileContent> {
    WalkDir::new(path)
        .max_depth(depth)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| FileContent::from_path(entry.path()).ok())
        .collect()
}

/// Returns a vector with dupesets
///
/// The vector of checked values needs to be mutable since the vector will be drained during
/// checking to avoid rechecking duplicates already found
///
/// # Examples
/// ```
///    use filedupes::*;
///    // get the files in the closest subfolder
///    let mut origs = get_files(&"./",2);
///
///    // compare to all files in all the subfolders
///    let mut checks = get_files(&"./", std::usize::MAX);
///
///    let dupesets = check_duplicates(&origs, &mut checks);
/// ```
pub fn check_duplicates<'a>( origs: &'a Vec<FileContent>, checks: &mut Vec<FileContent> ) ->
        Vec <DupeSet<'a>>
{
    let mut count = 1;
    let totlen = origs.len();

    let mut dupesets: Vec<DupeSet> = Vec::new();
    for orig in origs {
        print!("\r{}/{}: {:.1}% ", count, totlen, count as f64 /totlen as f64 * 100.0);
        io::stdout().flush().ok().expect("Could not flush stdout");
        count +=1;

        let duplicates: Vec<FileContent> = checks.drain_filter(|checked| *checked == *orig)
            .collect();
        //println!("{:?}", orig.path);

        if duplicates.len() > 0 {
            let set = DupeSet {
                original: orig,
                dupes: duplicates,
            };
            dupesets.push( set );
        }
    }
    println!("");
    dupesets
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::Bencher;

    #[test]
    fn find_files_ignore_errors() {
        let files = get_files("./testdata", 1000);

        assert_eq!(files.len(), 22, "got wrong amount of files");
    }

    fn find_dupes_of_original() {
        let oristr = "./testdata/original";
        let testdir = "./testdata";
        let originals = get_files(oristr, 0);
        let mut checks = get_files(testdir, 5);
        assert_eq!(originals.len(), 1, "More than one original file found"); // dupes will be a vector of one

        let dupes = check_duplicates(&originals, &mut checks);
        println!("dupes: {}", dupes.len());
        assert_eq!(dupes.len(), 1, "More than one dupe set found"); // dupes will be a vector of one
        assert_eq!(dupes.first().unwrap().dupes.len(), 3, "Wrong amount of duplicates"); // should be 3 copies of the original
    }

    fn find_dupes_of_original_limit() {
        let oristr = "./testdata/original";
        let testdir = "./testdata";
        let originals = get_files(oristr, 0);
        let mut checks = get_files(testdir, 1);
        assert_eq!(originals.len(), 1, "More than one original file found"); // dupes will be a vector of one

        let dupes = check_duplicates(&originals, &mut checks);
        println!("dupes: {}", dupes.len());
        assert_eq!(dupes.len(), 1, "More than one dupe set found"); // dupes will be a vector of one
        assert_eq!(dupes.first().unwrap().dupes.len(), 2, "Wrong amount of duplicates"); // should be 3 copies of the original
    }

    #[test]
    fn find_no_dupes() {
        let oristr = "./testdata/a";
        let testdir = "./testdata";
        let originals = get_files(oristr, 0);
        let mut checks = get_files(testdir, 5);
        assert_eq!(originals.len(), 1, "More than one original file found"); // dupes will be a vector of one

        let dupes = check_duplicates(&originals, &mut checks);
        println!("dupes: {}", dupes.len());
        assert_eq!(dupes.len(), 0, "dupeset was found"); // dupes will be a vector of one
    }

    #[bench]
    fn find_dupliates_of_one_file(b: &mut Bencher) {
        b.iter(|| {
            find_dupes_of_original()
        });
    }

}
