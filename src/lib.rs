extern crate rayon;
extern crate sha1;
extern crate walkdir;

use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub mod file;
pub mod hasher;
pub mod lazyfile;

pub use file::FileContent;
pub use hasher::Hasher;
pub use sha1::Sha1;
pub use std::fs::Metadata;
use walkdir::WalkDir;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identify_img_file() {
        let oristr = &String::from("./testdata/cats.jpg");
        let original = FileContent::from_path(&PathBuf::from(oristr)).expect("Error with original");
    }

    #[test]
    fn find_files_ignore_errors() {
        let files = get_files(&String::from("./testdata"), 1000);

        assert_eq!(files.len(), 22, "got wrong amount of files");
    }

    // #[test]
    // fn find_dupes_of_original() {
    //     let oristr = &String::from("./testdata/original");
    //     let testdir = &String::from("./testdata");
    //     let files = get_files_recursive_at(testdir).expect("Failed getting the files");
    //     let original = FileContent::from_path(&path::PathBuf::from(oristr)).unwrap();

    //     let comparefiles: Vec<Result<FileContent, std::io::Error>> =
    //         files.iter().map(|&x| FileContent::from_path(&x)).collect();

    //     let mut dupes: i64 = 0;
    //     for f in comparefiles {
    //         if f.unwrap() == original {
    //             dupes += 1;
    //         }
    //     }
    //     println!("dupes: {}", dupes);
    //     assert_eq!(dupes, 3);
    // }

    // #[test]
    // fn find_no_dupes() {
    //     let oristr = &String::from("./testdata/a");
    //     let testdir = &String::from("./testdata");
    //     let files = get_files_recursive_at(testdir).expect("Failed getting the files");
    //     let original =
    //         FileContent::from_path(&path::PathBuf::from(oristr)).expect("Error with original");
    // }

    // #[bench]
    // fn bench_new_duplicatereports(b: &mut Bencher) {
    //     b.iter(|| {});
    // }

}
