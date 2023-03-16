#![allow(unused)]
use clap::Parser;
use std::io::{self, Write};
#[derive(Parser)]
struct  Cli {
    pattern:String,
    path:std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    let stdout = io::stdout(); // get the global stdout entity;
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let result = std::fs::read_to_string(&args.path);
    let content2 =  match result {
        Ok(content) => { content },
        Err(error) =>{ panic!("Can't deal with {}, just exit here", error);},
    };
    for line in content2.lines() {
        if line.contains(&args.pattern) {
            println!("{}",line);
        }
    }

    //
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern:pattern,
    //     path:std::path::PathBuf::from(path),
    // };

}
