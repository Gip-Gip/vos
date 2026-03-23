pub mod assembler;
pub mod opcodes;

use std::fs::File;
use std::path::PathBuf;
use std::path::absolute;

use clap::Parser;

use crate::assembler::Assembler;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the assembly file to read from
    file: PathBuf,

    /// Output binary to write to
    #[arg(short, long)]
    out: PathBuf,

    /// Print debug info
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    if args.debug {
        colog::basic_builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        colog::init();
    }

    let in_file = File::open(args.file).unwrap();
    let out_file = File::create(args.out).unwrap();

    let assembler = Assembler::new(in_file, out_file);

    let result = assembler.assemble();

    if let Err(e) = result {
        log::error!("{}", e)
    }
}
