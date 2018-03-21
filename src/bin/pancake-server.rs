#[macro_use]
extern crate clap;
extern crate toml;
extern crate pancake;

use clap::App;
use clap::Arg;
use pancake::config::PancakeConfig;
use std::process;

const SAMPLE_CONFIG_ARG: &str = "print-sample-config";
const SINGLE_NODE_MODE: &str = "single-node-mode";

fn main() {
    let matches = App::new("Pancake")
        .version(crate_version!())
        .author("whatot whatot2@gmail.com")
        .about("A misc service by Rust")
        .arg(
            Arg::with_name("config")
                .short("C")
                .long("config")
                .value_name("FILE")
                .help("Set config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("addr")
                .short("A")
                .long("addr")
                .takes_value(true)
                .value_name("IP:PORT")
                .help("Set listening address"),
        )
        .arg(Arg::with_name(SAMPLE_CONFIG_ARG)
            .long(SAMPLE_CONFIG_ARG)
            .help("Print a sample config to stdout"),
        )
        .arg(Arg::with_name(SINGLE_NODE_MODE)
            .long(SINGLE_NODE_MODE)
            .help("Single node as a cluster")
        )
        .get_matches();

    if matches.is_present(SAMPLE_CONFIG_ARG) {
        let config = PancakeConfig::default();
        println!("{}", toml::to_string_pretty(&config).unwrap());
        process::exit(0);
    }

}
