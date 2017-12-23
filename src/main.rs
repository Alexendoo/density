use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::string::String;
use std::vec::Vec;

fn main() {
    let vec = &mut Vec::new();

    for arg in env::args().skip(1) {
        output_result(visit(&arg, vec), arg);
    }
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

fn output_result(result: io::Result<&Vec<u64>>, directory: String) {
    match result {
        Ok(res) => println!("{}\t{}", mean(res), directory),
        Err(err) => eprintln!("{}: {}", directory, err),
    }
}

fn mean(sizes: &Vec<u64>) -> u64 {
    let len = sizes.len() as u64;
    let sum: u64 = sizes.iter().sum();

    sum / len
}
