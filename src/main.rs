#![feature( vec_remove_item )]
#![feature(drain_filter)]
use std::{path, env, path::PathBuf };
use std::collections::BTreeMap;
use std::collections::btree_map::Entry as BTreeEntry;

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

    let mut searchpaths: Vec< path::PathBuf> = Vec::new();

    if settings.whole_dir {
        searchpaths = fdupe::get_files_recursive( searchpath.as_path());
    } else {
        searchpaths.push( searchpath );
    }

    //println!( "Checking {} files for duplicates", &searchfiles.len() );
    let comparepath: PathBuf = PathBuf::from( &settings.compare );
    let comparepaths = fdupe::get_files_recursive( comparepath.as_path() );

    //println!("against {} files", &comparefiles.len() );
    let mut searchfiles: Vec< FileContent> = searchpaths
        .par_iter()
        .map( |x| fdupe::FileContent::from_path( &x ) )
        .filter_map(|e| e.ok())
        .collect();

    if( searchpaths != comparepaths ) {
        let mut comparefiles: Vec< FileContent > = comparepaths
            .par_iter()
            .map( |x| fdupe::FileContent::from_path( &x ) )
            .filter_map(|e| e.ok())
            .collect();
        searchfiles.append( &mut comparefiles );
    }
    //println!("Searching!");


    let filecheckamount = searchfiles.len();
    let mut count = 0;
    let mut dupes = 0;
    let mut totalsize = 0;
    let mut sets = 0;
    let mut btree: BTreeMap<&FileContent, Vec<&FileContent> > = BTreeMap::new();
    for fc in &searchfiles {
            print!("\rFiles {}/{}",
                   count, filecheckamount);

        match btree.entry( fc ) {
            BTreeEntry::Vacant(e) => {
                // Seems unique so far
                e.insert( Vec::new() );
            },
            BTreeEntry::Occupied(mut e) => {
                // Found a dupe!
                dupes += 1;
                totalsize += fc.len();
                let filesets = e.get_mut();
                if filesets.len() == 0 {
                    sets += 1;
                }
                filesets.push(fc);
            },
        }
        count += 1;
    }
    println!();
    println!("{} duplicate files (in {} sets), occupying {} megabytes",
             dupes,
             sets,
             totalsize/(1024*1024) );
    // println!();
    // let mut filetotal = 0;
    // let mut sets = 0;
    // for r in dupesets.iter() {
    //     sets += 1;
    //     println!("{:?}",r.original.path);
    //     for d in r.dupes.iter() {
    //         println!("{:?}",d.path);
    //         filetotal += 1;
    //     }
    //     println!();
    // }
    // println!("duplicates: {}, sets: {}", filetotal, sets );



}

fn main() {

    let args: Vec<String> = env::args().collect();
    let args = parse_args( &args );
    run( &args );

}
