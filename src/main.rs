use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;

#[derive(Parser, Debug)]
#[command[version, about, long_about = None]]
struct Args {
    /// The key to use
    #[arg(short, long, default_value_t=String::from("secret"))]
    key: String,

    /// The length of the shift
    #[arg(short, long, default_value_t = 3)]
    shift: u8,

    /// The filename to encrypt
    #[arg(short, long)]
    filename: String,

    /// The filename to encrypt
    #[arg(short, long)]
    cipher: String,

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
    let cipher = &args.cipher.to_lowercase();

    let result = match &args.command {
        Some(Commands::Encrypt {}) => match cipher.as_str() {
            "caesar" => cipher_suite::caesar::encrypt(&contents, args.shift),
            "vignere" => cipher_suite::vignere::encrypt(&contents, args.key.as_str()),
            "hill" => cipher_suite::hill::encrypt(&contents, args.key.as_str()),
            _ => Err(String::from("Cipher not found")),
        },
        Some(Commands::Decrypt {}) => match cipher.as_str() {
            "caesar" => cipher_suite::caesar::decrypt(&contents, args.shift),
            "vignere" => cipher_suite::vignere::decrypt(&contents, args.key.as_str()),
            "hill" => cipher_suite::hill::encrypt(&contents, args.key.as_str()),
            _ => Err(String::from("Cipher not found")),
        },
        None => Err(String::from("Command not provided: encrypt or decrypt")),
    };
    match result {
        Ok(v) => println!("{}", v),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
