use clap::Parser;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::result;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();

    // let file = File::open(&args.path).expect("could not open file"); TEntei
    // let result = BufReader::new(file);

    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("Can't deal with {}, just exit here", error); } };
    println!("file content: {}", content);
    // for line in result.lines() {
    //     let line = line.expect("could not read line");
    //     if line.contains(&args.pattern){
    //         println!("{}", line);
    //     }
    // }

    for line in content.lines(){ //Basicamnete, ele le o arquivo linha por linha
        if line.contains(&args.pattern){ //Se a linha contiver o padrao, ele imprime a linha
            println!("{}", line);
        }
    }

    Ok(())
}
