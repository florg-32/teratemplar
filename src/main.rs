use std::{path::PathBuf, io::{Write}};
use anyhow::{Context, Result};

use clap::Parser;

mod formats;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, help("path to a file containing structured text conforming to the supplied format"))]
    input_file: Option<PathBuf>,
    #[arg(short, long, help("output path. Defaults to stdout"))]
    output_file: Option<PathBuf>,
    #[arg(short, long, help("path to a tera or jinja2 template"))]
    template: PathBuf,
    #[arg(short, long, help("supported: toml; defaults to input file ending if any"))]
    format: Option<String>

}

fn main() -> Result<()> {
    let args = Cli::parse();

    let format = match (&args.input_file, args.format) {
        (None, _) => None,
        (Some(_), Some(f)) => Some(f),
        (Some(i), None) => extension_as_string(i.clone()),
    };
    
    let context = if let Some(f) = format {
        let input = std::fs::read_to_string(args.input_file.unwrap()).with_context(|| "Could not open input file")?;
        formats::parse(&f, input)?
    }
    else {
        tera::Context::new()
    };

    let template_path = args.template.parent().context("Could not extract path from template file")?.to_string_lossy();
    let template_file = args.template.file_name().context("Could not extract template file name")?.to_string_lossy();
    let template_extension = args.template.extension().context("Could not extract template file extension")?.to_string_lossy();
    
    let tera = tera::Tera::new(&format!("{}/**/*.{}", template_path, template_extension)).unwrap();
    let output = tera.render(&template_file, &context)?;

    if let Some(path) = args.output_file {
        std::fs::write(path, output).with_context(|| "failed to write to output file")?;
    }
    else {
        std::io::stdout().write_all(output.as_bytes())?;
    }

    Ok(())
}

fn extension_as_string(p: PathBuf) -> Option<String> {
    p.extension().and_then(|e| e.to_str().and_then(|s| Some(s.to_string())))
}
