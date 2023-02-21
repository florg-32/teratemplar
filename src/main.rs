use std::{path::PathBuf, io::{Read, Write}};
use anyhow::{Context, Result};

use clap::Parser;

mod formats;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, help("path to a file containing structured text conforming to the supplied format. Defaults to stdin"))]
    input_file: Option<PathBuf>,
    #[arg(short, long, help("output path. Defaults to stdout"))]
    output_file: Option<PathBuf>,
    #[arg(short, long, help("path to a tera or jinja2 template"))]
    template: PathBuf,
    #[arg(short, long, help("supported: toml"))]
    format: String

}

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut input_file = String::new();
    if let Some(path) = args.input_file {
        std::fs::File::open(path)
            .with_context(|| "failed to open input file")?
            .read_to_string(&mut input_file)?;
    }
    else {
        std::io::stdin().read_to_string(&mut input_file)?;
    }

    let context = match args.format.as_str() {
        "toml" => {
            formats::parse_toml(input_file)?
        },
        _ => {
            eprintln!("Error: supported formats are toml");
            std::process::exit(1);
        }
    };

    let template_file: String = std::fs::read_to_string(args.template)?;
    let output = tera::Tera::one_off(template_file.as_str(), &context, false)?;
    if let Some(path) = args.output_file {
        std::fs::write(path, output).with_context(|| "failed to write to output file")?;
    }
    else {
        std::io::stdout().write_all(output.as_bytes())?;
    }

    Ok(())
}
