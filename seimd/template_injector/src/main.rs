mod decoration;
mod injector;
mod provider;

use crate::injector::Injector;
use crate::provider::Provider;
use clap::Parser;
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
    #[arg(help = "File to inject")]
    input: String,
}

fn main() -> Result<(), String> {
    let args = GlobalArgs::parse();

    let provider = Provider::new(args.seimd_path);
    let mut injector = Injector::new(provider);

    let input = fs::read_to_string(&args.input)
        .map_err(|e| format!("Error reading {}: {}", args.input, e.kind()))?;
    println!("{}", injector.inject(input)?);

    Ok(())
}
