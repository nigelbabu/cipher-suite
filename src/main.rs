use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command[version, about, long_about = None]]
struct Args {
    /// The key to use
    #[arg(short, long)]
    key: String,

    /// The filename to encrypt
    #[arg(short, long)]
    filename: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Encrypt {},
    Decrypt {},
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file = File::open(args.filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    
    let result = match &args.command {
        Some(Commands::Encrypt {}) => cipher_suite::vignere::encrypt(&contents, args.key.as_str()),
        Some(Commands::Decrypt {}) => cipher_suite::vignere::decrypt(&contents, args.key.as_str()),
        None => Err(String::from("Command not provided: encrypt or decrypt")),
    };
    match result {
        Ok(v) => println!("{}", v),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
