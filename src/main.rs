use std::{fs, path::PathBuf};

use clap::{Parser, ValueEnum};

mod cassylab;
mod spectralab;

#[derive(Clone, ValueEnum)]
enum Format {
    CassyLab,
    SpectraLab,
}

#[derive(clap::Parser)]
struct Cli {
    /// File to read from (must be xml)
    #[arg(short, long)]
    input: PathBuf,

    /// The format of the input file
    #[arg(short, long)]
    format: Format,
}

fn main() {
    let cli = Cli::parse();
    if !cli.input.exists() {
        panic!("File does not exist: {}", cli.input.display());
    }
    let input = fs::read_to_string(&cli.input).unwrap();
    match cli.format {
        Format::CassyLab => cassylab::convert(input, cli.input),
        Format::SpectraLab => spectralab::convert(input, cli.input),
    }
}
