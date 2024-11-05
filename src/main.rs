use anyhow::{Context, Result};
use clap::Parser;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
// use log::{info, warn};
// use env_logger;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {

    // env_logger::init();
    // info!("starting up");
    // warn!("oops, nothing implemented!");

    let args = Cli::parse();

    // let stdout = io::stdout();
    // let mut handle = stdout.lock();
    // writeln!(handle, "foo: {}", 42)?;

    let path = "test.txt";
    let content =
        std::fs::read_to_string(path).with_context(|| format!("could not read file `{}`", path))?;

    let progressbar = ProgressBar::new(100);
    progressbar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap());
    for _i in 0..100{
        progressbar.inc(1);
    }
    progressbar.finish_with_message("done");
    find_matche_word(&content, &args.pattern, &mut std::io::stdout());
    

    Ok(())
}

// fn progress_bar() {
    
// }

fn find_matche_word(content: &str, pattern: &str, mut writer: impl Write){
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matche_word("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}