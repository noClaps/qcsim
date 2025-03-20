mod computer;
mod parser;
mod qubit;

use std::{io::Read, process::exit};

use clap::Parser;
use clio::Input;
use parser::QCLang;

#[derive(Parser)]
struct Args {
    /// QC instructions file to be run
    #[clap(value_parser)]
    input: Input,
}

fn main() {
    let mut args = Args::parse();
    let mut input_str = String::new();
    match args.input.read_to_string(&mut input_str) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("[ERROR] Error reading file from input: {err}");
            exit(1);
        }
    }

    let mut qc = QCLang::new(input_str);
    qc.parse();
    qc.run();
}
