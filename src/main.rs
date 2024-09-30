use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;
use std::string::String;
use vigenere_cipher::Config;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });

    let file = File::open(config.filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    
    let mut result = String::new();
    if config.function == "encrypt" {
        result = vigenere_cipher::encrypt(&contents, "test");
    } else if config.function == "decrypt" {
        result = vigenere_cipher::decrypt(&contents, "test");
    } else {
        eprintln!("Function should be encrypt or decrypt")
    }
    println!("{}", result);
    Ok(())
}
