use clap::{Parser, Subcommand};
use kvs::{lsm::database::Database, Store};

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
    let mut store = Database::new("/tmp/kvs".into());

    match cli.command {
        Some(Commands::Set { key, value }) => {
            println!("run set, key:{}, value: {}", key, value);
            store.set(key.as_bytes(), value.as_bytes()).unwrap();
        }
        Some(Commands::Get { key }) => {
            let value = store.get(key.as_bytes()).unwrap().unwrap();
            println!("{:?}", std::str::from_utf8(value));
        }
        Some(Commands::Rm { key }) => {
            store.remove(key.as_bytes()).unwrap();
        }
        None => {}
    }
}
