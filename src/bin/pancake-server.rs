#[macro_use]
extern crate clap;
extern crate pancake;
extern crate toml;

use clap::App;
use clap::Arg;
use pancake::config::PancakeConfig;
use std::error::Error;
use std::fs::File;
use std::io::Read;
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

    let mut config = matches.value_of("config")
        .map_or_else(PancakeConfig::default, |path| {
            File::open(&path)
                .map_err::<Box<Error>, _>(|e| Box::new(e))
                .and_then(|mut f| {
                    let mut s = String::new();
                    f.read_to_string(&mut s)?;
                    let c = toml::from_str(&s)?;
                    Ok(c)
                })
                .unwrap_or_else(|e| {
                    println!("invalid config file, {:?}, {}", path, e);
                    process::exit(-1);
                })
        });

    if matches.is_present(SINGLE_NODE_MODE) {
        config.setup_single_node();
    }

    println!("final config:\n{:?}", config);
}
