use std::{path, fs, env, error::Error, path::PathBuf, process};

extern crate fdupe;
use fdupe::HashedPath;

fn get_files_recursive( _path: &path::Path ) -> Vec<path::PathBuf>
{
    let mut files: Vec<path::PathBuf> = Vec::new();

    let paths = fs::read_dir( _path ).unwrap();

    for path in paths {
        match path {
            Ok(d) => {
                if !d.path().is_dir() {
                    files.push( PathBuf::from( fs::canonicalize( d.path() ).expect("") ) );
                }
                else {
                    let mut dirfiles = get_files_recursive( &d.path() );
                    files.append( &mut dirfiles );
                }
            },
            Err(e) => {
                println!("error: {}", e.description() );
            }
        }
    }
    return files;
}

struct Settings{
    search: String,
    compare: String,
    whole_dir: bool,
}

fn parse_args( _args: &Vec<String> ) -> Settings
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
        println!("not implemented yet");
        process::exit(1);
    }
    return settings;
}

fn run( settings: &Settings) {
    //set to find dupes for
    let searchpath: PathBuf = PathBuf::from( &settings.search);
    let searchfiles = get_files_recursive( searchpath.as_path());

    // create set to compare to
    let comparepath: PathBuf = PathBuf::from( &settings.compare );
    let comparefiles = get_files_recursive( comparepath.as_path() );

    let searchfiles: Vec< Result<HashedPath, std::io::Error>> = searchfiles.iter()
        .map( |x| fdupe::hash_path( &x ) )
        .collect();

    let comparefiles: Vec< Result<HashedPath, std::io::Error>> = comparefiles.iter()
        .map( |x| fdupe::hash_path( &x ) )
        .collect();

    let searchfiles: Vec< &fdupe::HashedPath > = searchfiles.iter().flat_map( |x| x).collect();
    let comparefiles: Vec< &fdupe::HashedPath > = comparefiles.iter().flat_map( |x| x).collect();

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
