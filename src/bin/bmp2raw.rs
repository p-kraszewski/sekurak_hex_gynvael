use clap::Parser;
use env_logger::Env;
use eyre::Result;
use log::{debug, error};

use sekurak_hex_gynvael::image_codec as ic;

#[derive(Parser)]
#[command(name = "bmp2png")]
#[command(author = "Pyth0n")]
#[command(version = "1.0")]
#[command(about = "Simple UNZIP implementation", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "data/sing_scape.broken.bmp")]
    /// Input ZIP file
    in_file: String,

    /// Directory to unpack to
    #[arg(short, long, default_value = "data/sing_scape.raw")]
    out_file: String,

    /// Turn debugging on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let log_level = match args.debug {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level))
        .format_timestamp(None)
        .init();

    let img = match ic::bmp::read_bmp(&args.in_file) {
        Ok(img) => img,
        Err(e) => {
            error!("{}", e);
            return Ok(());
        }
    };

    ic::raw::write_raw(&args.out_file, &img)?;

    Ok(())
}
