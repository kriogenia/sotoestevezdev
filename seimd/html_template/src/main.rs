use std::error::Error;
use std::fs;
use clap::Parser;
use regex::Regex;
use seimd::parser::SeimdParser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(
        short = 'i',
        long = "injectables",
        help = "Path where the SEimd files are stored",
    )]
    seimd_path: String,
    #[arg(
        short = 'o',
        long = "output",
        help = "Path to output the processed HTMLs",
    )]
    output_path: String,
    #[arg(help = "List of files to read")]
    files: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let parser = SeimdParser::new();
    let regex = Regex::new(r"\{{2}(.*)}{2}").unwrap();

    for file in args.files.iter() {
        let input = fs::read_to_string(file)?;
        let file_to_inject = regex.captures(&input).unwrap().get(1).unwrap().as_str().trim();
        let file_path = format!("{}/{}", args.seimd_path, file_to_inject);
        let injectable_content = fs::read_to_string(dbg!(file_path))?;
        let parsed = parser.parse(injectable_content);
        dbg!(parsed);
    }
    Ok(())
}
