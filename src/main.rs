
#[macro_use]
extern crate prometheus;
extern crate hyper;
extern crate csv;
extern crate toml;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod exporter;
mod softether_reader;

use std::env;
use exporter::{Config, Exporter};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!( "Usage: softether-exporter [config_file]" );
        return
    }

    let config = match Config::from_file( &args[1] ) {
        Ok ( x ) => x,
        Err( x ) => { println!( "Config ( {} ) read failed: {}", &args[1], x ); return },
    };

    match Exporter::start( config ) {
        Ok ( _ ) => (),
        Err( x ) => { println!( "Server error failed: {}", x ); return },
    }
}

