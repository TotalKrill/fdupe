#![feature( vec_remove_item )]
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

    // create set of files to compare to
    let comparepath: PathBuf = PathBuf::from( &settings.compare );
    let comparefiles = fdupe::get_files_recursive( comparepath.as_path() );

    println!("against {} files",
             &comparefiles.len() );

    let searchfiles: Vec< Result<FileContent, std::io::Error>> = searchfiles.par_iter()
        .map( |x| fdupe::FileContent::from_path( &x ) )
        .collect();

    let comparefiles: Vec< Result<FileContent, std::io::Error>> = comparefiles.par_iter()
        .map( |x| fdupe::FileContent::from_path( &x ) )
        .collect();

    let searchfiles: Vec< fdupe::FileContent > = searchfiles.into_iter()
        .flat_map( |x| x)
        .collect();

    let mut comparefiles: Vec< fdupe::FileContent > = comparefiles.into_iter()
        .flat_map( |x| x)
        .collect();

    println!();

}

fn main() {

    let args: Vec<String> = env::args().collect();
    let args = parse_args( &args );
    run( &args );

}
