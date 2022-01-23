use clap::{AppSettings, Subcommand, Parser};
use kvs::KvStore;
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Set the value of a string key to a string
    Set {
        /// A string key
        key: String,
        /// The string value of the key
        value: String },
    /// Get the string value of a given string key
    Get {
        /// A string key
        key: String,
    },
    /// Remove a given key
    Rm {
        /// A string key
        key: String,
    },
}

fn main() {

    // https://github.com/clap-rs/clap/blob/v3.0.10/examples/tutorial_derive/03_04_subcommands.rs
    let cli = Cli::parse();
    let mut kvstore = KvStore::new();

    match &cli.command {
        Commands::Get { key } => {
            println!("{:?}", kvstore.get(key.to_string()));
            exit(0);
        }
        Commands::Set {key, value} => {
            kvstore.set(key.to_string(), value.to_string());
            exit(0);
        }
        Commands::Rm {key} => {
            kvstore.remove(key.to_string());
            exit(0);
        }
    }
}
