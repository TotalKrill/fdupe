extern crate clap;
extern crate walkdir;
extern crate pretty_bytes;
extern crate dupefiles;

use clap::{App, Arg};
use pretty_bytes::converter::convert;
use std::fs;

use dupefiles::*;

struct AppSettings {
    originals_folder: String,
    original_depth: usize,
    checkfolder: String,
    check_depth: usize,
    delete: bool,
    noprint: bool,
    summarize: bool,
}

fn parse_args() -> AppSettings {
    let maxstr = &std::usize::MAX.to_string();

    let matches = App::new("ripdupes")
        .version("1.0")
        .arg(
            Arg::with_name("originals-path")
                .help("The path of the files to be treated as orginals")
                .long("originals-path")
                .required(true)
                .short("o")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("check-path")
                .help("Folder to check for duplicates in")
                .long("check-path")
                .required(true)
                .short("c")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("original-depth")
                .help("Depth of originals directory traversal")
                .long("odepth")
                .takes_value(true)
                .default_value(maxstr),
        )
        .arg(
            Arg::with_name("check-depth")
                .help("Depth of compare to directory traversal")
                .long("cdepth")
                .takes_value(true)
                .default_value(maxstr),
        )
        .arg(
            Arg::with_name("quiet")
                .help("Dont print every output")
                .short("q")
                .long("quiet"),
        )
        .arg(
            Arg::with_name("delete")
                .help("Delete the duplicates")
                .long("delete"),
        )
        .get_matches();

    let opath = matches
        .value_of("originals-path")
        .expect("invalid pathname");

    let cpath = matches.value_of("check-path").expect("invalid pathname");

    let odepth: usize = matches
        .value_of("original-depth")
        .expect("invalid depth")
        .parse()
        .expect("invalid depth");

    let cdepth: usize = matches
        .value_of("check-depth")
        .expect("invalid depth")
        .parse()
        .expect("invalid depth");

    let del = matches.is_present("delete");
    let sum = true;
    let noprint = matches.is_present("no-print");

    AppSettings {
        originals_folder: opath.to_string(),
        original_depth: odepth,
        checkfolder: cpath.to_string(),
        check_depth: cdepth,
        summarize: sum,
        noprint: noprint,
        delete: del,
    }
}


fn main() {
    let args = parse_args();

    let mut origs = get_files(&args.originals_folder, args.original_depth);
    origs.sort();
    origs.dedup();

    let mut checks = get_files(&args.checkfolder, args.check_depth);
    println!("{} Originals checked against {} files", origs.len(), checks.len());
    let dupesets = check_duplicates(&origs, &mut checks);

    if !args.noprint {

        for dupe in &dupesets {
            println!("=========");
            println!("Original: {:?}", dupe.original.path);
            for d in &dupe.dupes {
                println!("Duplicate: {:?}", d.path);
            }
        }
        println!("=========");
    }

    if args.delete {
        let mut totdupes = 0;
        let mut totsize = 0;
        for dupe in &dupesets {
            for d in &dupe.dupes {
                totsize += d.len();
                totdupes += 1;
                let res = fs::remove_file( &d.path );
                if let Err(res) = res {
                    println!("Error removing file: {}", res);
                }
            }
        }
        println!("{} originals had {} dupes occupying {}, that were removed",
                 dupesets.len(), totdupes, convert( totsize as f64 ));
    }
    //Delete command also prints out sum
    if args.summarize && !args.delete {
        let mut totdupes = 0;
        let mut totsize = 0;
        for dupe in &dupesets {
            for d in &dupe.dupes {
                totsize += d.len();
                totdupes += 1;
            }
        }
        println!("{} originals has {} dupes occupying {}",
                 dupesets.len(), totdupes, convert( totsize as f64 ));
    }
}
