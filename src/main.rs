use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::vec::Vec;

fn main() {
    let vec = &mut Vec::new();
    let path = env::current_dir().unwrap();

    let res = visit(&path, vec);

    println!("{} bytes/file", mean(res.unwrap()));
}

fn mean(sizes: &Vec<u64>) -> u64 {
    let len = sizes.len() as u64;
    let sum: u64 = sizes.iter().sum();

    sum / len
}

fn visit<'a>(parent: &Path, sizes: &'a mut Vec<u64>) -> io::Result<&'a Vec<u64>> {
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
