extern crate getopts;

use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::string::String;
use std::vec::Vec;



fn main() {
    let mut opts = getopts::Options::new();
    opts.optflag("h", "human-readable", "display human readable sizes (e.g. 1G, 50K)");
    opts.optflag("", "si", "human-readable output (-h) with powers of 1000 rather than 1024");

    opts.optflag("", "help", "display this help message");
    opts.optflag("", "version", "display the version number");

    let args: Vec<String> = env::args().collect();

    match opts.parse(&args[1..]) {
        Ok(matches) => run(args, matches, opts),
        Err(failure) => println!("{}", failure),
    }
}

fn run(args: Vec<String>, matches: getopts::Matches, opts: getopts::Options) {
    if matches.opt_present("help") {
        print_usage(&args[0], opts);
        return;
    }

    if matches.opt_present("version") {
        println!("density v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    for arg in &matches.free {
        let vec = &mut Vec::new();

        match visit(&arg, vec) {
            Ok(result) => print_result(result, arg, &matches),
            Err(error) => eprintln!("{}: {}", arg, error),
        }
    }
}

fn print_usage(program: &str, opts: getopts::Options) {
    let mut brief = opts.short_usage(program);
    brief.push_str(" [FILE]...");

    println!("{}", opts.usage(&brief));
}

fn visit<'a, P: AsRef<Path>>(
    parent: &P,
    sizes: &'a mut Vec<u64>,
) -> io::Result<&'a Vec<u64>> {
    let metadata = fs::metadata(parent)?;

    if metadata.is_dir() {
        for entry in fs::read_dir(parent)? {
            let path = entry?.path();

            visit(&path, sizes)?;
        }
    } else {
        sizes.push(metadata.len());
    }

    Ok(sizes)
}

const SUFFIXES: [&str; 9] = ["", "K", "M", "G", "T", "P", "E", "Z", "Y"];

fn print_result(
    result: &Vec<u64>,
    directory: &str,
    matches: &getopts::Matches,
) {
    let count = mean(result);

    let si = matches.opt_present("si");
    if si || matches.opt_present("human-readable")  {
        let base: u64 = if si {
            1000
        } else {
            1024
        };

        let index = f64::log(count as f64, base as f64).floor();

        let power = base.pow(index as u32);

        println!(
            "{}{}\t{}",
            count / power,
            SUFFIXES[index as usize],
            directory
        );
    } else {
        println!("{}\t{}", count, directory);
    }
}

fn mean(sizes: &Vec<u64>) -> u64 {
    let len = sizes.len() as u64;
    let sum: u64 = sizes.iter().sum();

    sum / len
}
