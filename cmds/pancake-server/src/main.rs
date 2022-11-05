extern crate clap;
extern crate serde;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod profile;

use std::{error::Error, fs::File, io::Read, path::PathBuf, process};

use clap::{Parser, Subcommand};

use crate::profile::ProfileOpt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Set listening address
    #[arg(short, long, value_name = "IP:PORT")]
    addr: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Print a sample config to stdout
    #[arg(long)]
    print_same_config: bool,

    /// Single node as a cluster
    #[arg(long)]
    single_node_mode: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.print_same_config {
        let config = ProfileOpt::default();
        println!("{}", toml::to_string_pretty(&config).unwrap());
        process::exit(0);
    }

    let mut config = cli.config.map_or_else(ProfileOpt::default, |path| {
        File::open(&path)
            .map_err::<Box<dyn Error>, _>(|e| Box::new(e))
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

    if cli.single_node_mode {
        config.setup_single_node();
    }

    println!("final config:\n{:?}", config);
}
