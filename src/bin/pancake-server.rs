#[macro_use]
extern crate clap;
extern crate toml;
extern crate pancake;

use clap::App;
use clap::Arg;
use pancake::config::PancakeConfig;
use std::process;

const SAMPLE_CONFIG_ARG: &str = "print-sample-config";

fn main() {
    let matches = App::new("Pancake")
        .version(crate_version!())
        .arg(Arg::with_name(SAMPLE_CONFIG_ARG)
            .long(SAMPLE_CONFIG_ARG)
            .help("Print a sample config to stdout"),
        )
        .get_matches();

    if matches.is_present(SAMPLE_CONFIG_ARG) {
        let config = PancakeConfig::default();
        println!("{}", toml::to_string_pretty(&config).unwrap());
        process::exit(0);
    }
}
