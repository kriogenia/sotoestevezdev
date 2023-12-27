mod provider;

use crate::provider::Provider;
use clap::Parser;
use regex::Regex;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct GlobalArgs {
    #[arg(
        short = 'i',
        long = "injectables",
        help = "Path where the SEimd files are stored"
    )]
    seimd_path: String,
    #[arg(short, long = "output", help = "Path to output the processed HTMLs")]
    output_path: String,
    #[arg(help = "List of files to read")]
    files: Vec<String>,
}

fn read_file(file: &str) -> Result<String, String> {
    fs::read_to_string(file).map_err(|e| format!("Error reading {file}: {}", e.kind()))
}

fn main() -> Result<(), String> {
    let args = GlobalArgs::parse();
    let mut provider = Provider::new(args.seimd_path);
    let regex = Regex::new(r"\{{2}(.*)}{2}").unwrap();

    for file in args.files.iter() {
        let input = read_file(file)?;
        let file_to_inject = regex
            .captures(&input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .trim();
        let parsed = provider.get(file_to_inject);
        dbg!(parsed.unwrap().html);
    }
    Ok(())
}
