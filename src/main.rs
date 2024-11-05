use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};
use indicatif::{style, ProgressBar, ProgressStyle};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "foo: {}", 42)?;

    let path = "test.txt";
    let content =
        std::fs::read_to_string(path).with_context(|| format!("could not read file `{}`", path))?;

    let progressbar = indicatif::ProgressBar::new(100);
    progressbar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap());
    for _i in 0..100{
        //progress_bar();
        //pb.println(format!("[+] finished #{}", i));
        progressbar.inc(1);
    }
    progressbar.finish_with_message("done");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

// fn progress_bar() {
    
// }