mod decoration;
mod injector;
mod provider;

use crate::injector::Injector;
use crate::provider::Provider;
use clap::Parser;
use std::fs;
use crate::decoration::DecoratorBuilder;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct GlobalArgs {
    #[arg(help = "File to inject")]
    input: String,
    #[arg(
        short = 'i',
        long = "injectables",
        help = "Path where the SEimd files are stored"
    )]
    seimd_path: String,
    #[arg(default_value = "false", long, help = "Wraps in parent if present in the file metadata")]
    parent: bool,
    #[arg(default_value = "false", long, help = "Injects the legend if present in the metadata")]
    legend: bool,
}

fn main() -> Result<(), String> {
    let args = GlobalArgs::parse();

    let provider = Provider::new(args.seimd_path);
    let decorators = DecoratorBuilder::new().legend(args.legend).parent(args.parent).build();

    let mut injector = Injector::new(provider, decorators);

    let input = fs::read_to_string(&args.input)
        .map_err(|e| format!("Error reading {}: {}", args.input, e.kind()))?;
    println!("{}", injector.inject(input)?);

    Ok(())
}
