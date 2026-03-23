mod vinteng;

use std::fs::File;
use std::path::PathBuf;

use clap::Parser;

use crate::vinteng::VintEngine;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output binary to write to
    #[arg(long)]
    vint: Option<PathBuf>,

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

    if let Some(vint_filename) = args.vint {
        let vint_file = File::open(vint_filename).unwrap();

        let mut engine = VintEngine::new(vint_file, 128).unwrap();

        let result = engine.run();

        if let Err(e) = result {
            log::error!("Fatal error: {}", e);
        } else {
            log::info!("Exited cleanly");
        }

        if args.debug {
            log::debug!("{:#?}", engine);
        }
    }
}
