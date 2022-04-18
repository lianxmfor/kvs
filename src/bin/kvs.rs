use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set the value of a string key to a string
    Set {
        /// A string key
        #[clap(short, long)]
        key: String,

        /// The string value of the key
        #[clap[short, long]]
        value: String,
    },

    /// Get the string value of a given string key
    Get {
        /// Get the string value of a given string key
        #[clap(short, long)]
        key: String,
    },

    Rm {
        /// Remove a given key
        #[clap(short, long)]
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Set { key, value }) => {
            println!("run set, key:{}, value: {}", key, value);
        }
        Some(Commands::Get { key }) => {
            println!("run get, key: {}", key);
        }
        Some(Commands::Rm { key }) => {
            println!("run rm, key: {}", key);
        }
        None => {}
    }
}
