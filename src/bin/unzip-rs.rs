use clap::Parser;
use env_logger::Env;
use eyre::{Result, WrapErr};
use log::debug;
use sekurak_hex_gynvael::{files, unzip};
use std::fmt::Debug;
use std::path::Path;

#[derive(Parser)]
#[command(name = "unzip-rs")]
#[command(author = "Pyth0n")]
#[command(version = "1.0")]
#[command(about = "Simple UNZIP implementation", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "data/in.zip")]
    /// Input ZIP file
    in_file: String,

    /// Directory to unpack to
    #[arg(short, long, default_value = "out")]
    out_dir: String,

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
    unzip(&args.in_file, &args.out_dir)
}

pub fn unzip(in_file_name: &str, out_dir_name: &str) -> Result<()> {
    let mut zip = unzip::ZipFile::open(in_file_name, out_dir_name)?;
    debug!("Opened {in_file_name}");
    let eocd = zip.find_end_of_central_directory()?;
    let dir_entries = zip.parse_central_directory(&eocd)?;

    for entry in dir_entries {
        zip.unpack_file(&entry)?;
    }
    Ok(())
}
