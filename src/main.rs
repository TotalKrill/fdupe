#![feature( vec_remove_item )]
#![feature(drain_filter)]
use std::{path, env, path::PathBuf };

extern crate fdupe;
use fdupe::*;

extern crate rayon;
use rayon::prelude::*;


struct Settings{
    search: String,
    compare: String,
    whole_dir: bool,
}
mod file;
use fdupe::FileContent;


fn parse_args( _args: &[String]) -> Settings
{
    let mut settings: Settings = Settings {
        search: String::from("."),
        compare: String::from("."),
        whole_dir: true,
    };

    if _args.len() < 2  {
        // Search for dupes in current folder
        println!("Finding dupes in current folder:");
    } else if _args.len() < 3 {
        // assume we want to compare current directory with the given path
        settings.compare = _args[1].clone();
        let res = path::PathBuf::from( &settings.compare );
        if !res.is_dir() {
            settings.whole_dir = false;
        }
    } else {
        // assume we want to compare arg 1 directory with the other paths
        settings.search = _args[1].clone();
        settings.compare = _args[2].clone();
    }
    settings
}

fn run( settings: &Settings) {

    //set to find dupes for
    let searchpath: PathBuf = PathBuf::from( &settings.search );

    let mut searchfiles: Vec< path::PathBuf> = Vec::new();

    if settings.whole_dir {
        searchfiles = fdupe::get_files_recursive( searchpath.as_path());
    } else {
        searchfiles.push( searchpath );
    }

    println!( "Checking {} files for duplicates",
             &searchfiles.len() );
    // let comparefiles = Vec::new();
    // if  &settings.search == &settings.compare {
    //     let comparefiles = searchfiles.clone();
    // } else {
        // create set of files to compare to
        let comparepath: PathBuf = PathBuf::from( &settings.compare );
        let comparefiles = fdupe::get_files_recursive( comparepath.as_path() );
    //  }

    println!("against {} files", &comparefiles.len() );

    let searchfiles: Vec< FileContent> = searchfiles.par_iter()
        .map( |x| fdupe::FileContent::from_path( &x ) )
        .filter_map(|e| e.ok())
        .collect();

    let mut comparefiles: Vec< FileContent > = comparefiles.par_iter()
        .map( |x| fdupe::FileContent::from_path( &x ) )
        .filter_map(|e| e.ok())
        .collect();

    let mut dupesets = Vec::with_capacity(searchfiles.len());
    let mut count = 0;

    // do a while loop here agains size of searchfiles,
    // then remove files from searchfiles
    for original in &searchfiles {
        count = count + 1;
        print!("\rFiles {}/{}",
            count,
            searchfiles.len());

        let mut dupes: Vec< FileContent > = comparefiles
            .drain_filter( |x| &x == &original )
            .collect();

        dupesets.push( dupes );
        }

}

fn main() {

    let args: Vec<String> = env::args().collect();
    let args = parse_args( &args );
    run( &args );

}
