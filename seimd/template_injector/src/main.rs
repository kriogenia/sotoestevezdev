mod decoration;
mod injector;
mod provider;

use crate::decoration::DecoratorBuilder;
use crate::injector::Injector;
use crate::provider::Provider;
use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(name = "SEimd Template Injector")]
#[command(about, author, version)]
struct GlobalArgs {
    #[arg(help = "File to inject", value_name = "INPUT_FILE")]
    input: String,
    #[arg(
        short = 'i',
        long = "injectables",
        help = "Path where the SEimd files are stored"
    )]
    seimd_path: String,
    #[arg(
        default_value = "false",
        long,
        help = "Wraps in parent if present in the file metadata"
    )]
    parent: bool,
    #[arg(
        default_value = "false",
        long,
        help = "Injects the summary if present in the metadata"
    )]
    summary: bool,
}

fn main() -> Result<(), String> {
    let args = GlobalArgs::parse();

    let provider = Provider::new(args.seimd_path);
    let decorators = DecoratorBuilder::new()
        .summary(args.summary)
        .parent(args.parent)
        .build();

    let mut injector = Injector::new(provider, decorators);

    let input = fs::read_to_string(&args.input)
        .map_err(|e| format!("Error reading {}: {}", args.input, e.kind()))?;
    println!("{}", injector.inject(input)?);

    Ok(())
}
