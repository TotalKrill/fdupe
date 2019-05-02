# filedupes

Library that can be used to lazily (not load entire file directly) check for duplicates on two vectors of filecontent

Uses file hashing code from [dupe-krill](https://github.com/kornelski/dupe-krill)

## Example
```
use filedupes::*;

fn main() {
    // get the files in the closest subfolder
    let mut origs = get_files(&"./",2);

    // just do a quick removal of all original duplicates, sort will sort on size
    origs.sort();
    origs.dedup();

    // compare to all files in all the subfolders
    let mut checks = get_files(&"./", std::usize::MAX);
    println!("{} Originals checked against {} files", origs.len(), checks.len());
    let dupesets = check_duplicates(&origs, &mut checks);

    println!( "Amount of duplicates of first {}", dupesets.first().unwrap().dupes.len() );
}
```

## Binary

the crate contains a binary called ripdupe which is a small software tool that can check for duplicates
and either remove them or just show and summarize them.

Example searching for duplicates of cats.jpg

    cargo run --release -- -o testdata/cats.jpg -c testdata/

A small software tool that can compare one set of file, to find duplicates in another set. It has
some differences to similar tools due to the fact it can limit the traversal depth in folders, thus
allow to only change or include files that not in subfolders.

Example only taking the toplevel files in a folder, and looking for duplicates in the closest subfolders

    cargo run --release -- -o testdata/ --odepth 1 -c testdata/ --cdepth 2

## Warning

I take no responsibility of files lost, since this program is specifically created to delete
duplicate files

