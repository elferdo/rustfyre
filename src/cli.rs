use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    pub iterations: u32,

    #[arg(short, long)]
    pub output_filename: PathBuf,

    #[arg(short, allow_negative_numbers(true))]
    pub a: f64,

    #[arg(short, allow_negative_numbers(true))]
    pub b: f64,

    #[arg(short, allow_negative_numbers(true))]
    pub c: f64,

    #[arg(short, allow_negative_numbers(true))]
    pub d: f64,
}
