mod exporter;
mod softether_reader;

use crate::exporter::{Config, Exporter};
use anyhow::Error;
use std::env;
use std::path::PathBuf;
use structopt::{clap, StructOpt};

// -------------------------------------------------------------------------------------------------
// Opt
// -------------------------------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
#[structopt(setting(clap::AppSettings::DeriveDisplayOrder))]
pub struct Opt {
    /// Address on which to expose metrics and web interface.
    #[structopt(long = "web.listen-address", default_value = ":9411")]
    pub listen_address: String,

    /// Config file.
    #[structopt(long = "config.file")]
    pub config: PathBuf,

    /// Show verbose message
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

// -------------------------------------------------------------------------------------------------
// Main
// -------------------------------------------------------------------------------------------------

fn run() -> Result<(), Error> {
    let opt = Opt::from_args();

    let config = Config::from_file(&opt.config)?;

    Exporter::start(config, &opt.listen_address, opt.verbose)?;
    Ok(())
}

fn main() {
    if let Err(x) = run() {
        println!("{}", x);
    }
}
