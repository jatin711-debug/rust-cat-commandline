use std::error::Error;
use clap::{Arg,App};
use std::fs::File;
use std::io::{self,BufRead,BufReader};

#[derive(Debug)]
pub struct Config {
    files:Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T,Box<dyn Error>>;

pub fn run(config:Config) -> MyResult<()>{
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}:{}",filename,err),
            Ok(_) => println!("Opened {} Success",filename),
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config>{
    let matches = App::new("catr")
        .version("0.0.1")
        .author("Jatin Mahajan")
        .about("Rust cat version")
        .arg(
                Arg::with_name("files")
                .value_name("File")
                .help("Input File Name")
                .multiple(true)
                .default_value(".")
            )
        .arg(
                Arg::with_name("number")
                .short("n")
                .long("number")
                .takes_value(false)
                .conflicts_with("number_nonblank")
                .help("Number Line")
            )
        .arg(
                Arg::with_name("number_nonblank")
                .long("number-nonblank")
                .short("b")
                .help("Number nonblank lines")
                .takes_value(false)
            )
        .get_matches();

    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank") 
    })
}

