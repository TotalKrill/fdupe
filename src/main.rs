use std::{path, env, path::PathBuf };

extern crate fdupe;
use fdupe::FileIdentification;


struct Settings{
    search: String,
    compare: String,
    whole_dir: bool,
}

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
    let searchfiles = fdupe::get_files_recursive( searchpath.as_path());

    println!( "Checking {} files for duplicates",
             &searchfiles.len() );

    // create set to compare to
    let comparepath: PathBuf = PathBuf::from( &settings.compare );
    let comparefiles = fdupe::get_files_recursive( comparepath.as_path() );

    println!("against {} files",
             &comparefiles.len() );

    let searchfiles: Vec< Result<FileIdentification, std::io::Error>> = searchfiles.iter()
        .map( |x| fdupe::FileIdentification::new( &x ) )
        .collect();

    println!("Hashed Searchfiles");

    let comparefiles: Vec< Result<FileIdentification, std::io::Error>> = comparefiles.iter()
        .map( |x| fdupe::FileIdentification::new( &x ) )
        .collect();

    println!("Hashed Comparefiles");


    let searchfiles: Vec< &fdupe::FileIdentification > = searchfiles.iter().flat_map( |x| x).collect();
    let comparefiles: Vec< &fdupe::FileIdentification > = comparefiles.iter().flat_map( |x| x).collect();

    for sfile in &searchfiles {
        for cfile in &comparefiles {
            if ( sfile != cfile ) &&
                ( sfile.hash == cfile.hash )
                {
                    println!("Original {}", sfile );
                    println!("Dupe {}", cfile );
                }
        }
    }

    // for file in &files {
    //     match file {
    //         Ok(v) => println!("{}", v ),
    //         Err(e) => println!("Error: {}", e),
    //     }
    // }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let args = parse_args( &args );
    run( &args );

}
