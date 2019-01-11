mod exporter;
mod softether_reader;

use crate::exporter::{Config, Exporter};
use std::env;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static BUILD_TIME: Option<&'static str> = option_env!("BUILD_TIME");
static GIT_REVISION: Option<&'static str> = option_env!("GIT_REVISION");

fn main() {
    let version_info = if BUILD_TIME.is_some() {
        format!(
            "  version   : {}\n  revision  : {}\n  build time: {}\n",
            VERSION,
            GIT_REVISION.unwrap_or(""),
            BUILD_TIME.unwrap()
        )
    } else {
        format!("  version: {}\n", VERSION)
    };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: softether_exporter [config_file]");
        println!("\n{}", version_info);
        return;
    }

    let config = match Config::from_file(&args[1]) {
        Ok(x) => x,
        Err(x) => {
            println!("Config ( {} ) read failed: {}", &args[1], x);
            return;
        }
    };

    match Exporter::start(config) {
        Ok(_) => (),
        Err(x) => {
            println!("Server start failed: {}", x);
            return;
        }
    }
}
