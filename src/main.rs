use include_dir::{include_dir, Dir, DirEntry};
use quicli::prelude::*;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use structopt::StructOpt;

// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
    /// Name
    name: String,
}

const TEMPLATE_DIR: Dir = include_dir!("./templates");

fn main() -> CliResult {
    let args = Cli::from_args();

    for entry in TEMPLATE_DIR.find("lite/**/*").unwrap() {
        match entry {
            DirEntry::File(f) => {
                let path = f.path();
                let contents = f.contents();

                let mut file = File::create(&path).unwrap();
                file.write_all(contents).unwrap();
            }
            DirEntry::Dir(dir) => {
                let path = dir.path();
                create_dir_all(&path).unwrap();
            }
        }
    }

    Ok(())
}
