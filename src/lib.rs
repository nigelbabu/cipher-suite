pub struct Config {
    pub filename: String,
    pub function: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();
        let function = args[2].clone().to_lowercase();
        Ok(Config { filename, function })
    }
}

// Used to encrypt
fn shift_right(input: u8, shift: u8, start: u8) -> u8 {
    ((input - start + shift) % 26) + start
}

// Used to decrypt
fn shift_left(input: u8, shift: u8, start: u8) -> u8 {
    let result = input - shift;
    if result < start {
        return result + 26;
    }
    result
}

pub fn encrypt(input: &str, key: &str) -> String {
    let mut result: Vec<u8> = Vec::new();
    let key = key.as_bytes();
    let mut i = 0;
    for letter in input.chars() {
        result.push(match letter {
            l @ 'a'..='z' => {
                let res = shift_right(l as u8, (key[i % key.len()]) - 'a' as u8, 'a' as u8);
                i += 1;
                res
            }
            l @ 'A'..='Z' => {
                let res = shift_right(l as u8, (key[i % key.len()]) - 'a' as u8, 'A' as u8);
                i += 1;
                res
            }
            l @ _ => l as u8,
        });
    }
    let secret = match String::from_utf8(result) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    secret
}

pub fn decrypt(input: &str, key: &str) -> String {
    let mut result: Vec<u8> = Vec::new();
    let key = key.as_bytes();
    let mut i = 0;
    for letter in input.chars() {
        result.push(match letter {
            l @ 'a'..='z' => {
                let res = shift_left(l as u8, (key[i % key.len()]) - 'a' as u8, 'a' as u8);
                i += 1;
                res
            }
            l @ 'A'..='Z' => {
                let res = shift_left(l as u8, (key[i % key.len()]) - 'a' as u8, 'A' as u8);
                i += 1;
                res
            }
            l @ _ => l as u8,
        });
    }
    let secret = match String::from_utf8(result) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_simple_test() {
        let result = encrypt("This is a test", "test");
        assert_eq!(result, "Mlal bw s mxwl");
    }
    #[test]
    fn decrypt_simple_test() {
        let result = decrypt("Mlal bw s mxwl", "test");
        assert_eq!(result, "This is a test");
    }
    #[test]
    fn encrypt_test() {
        let result = encrypt("abcdefghijklmnopqrstuvwxyz", "test");
        assert_eq!(result, "tfuwxjyabncefrgijvkmnzoqrd");
    }
    #[test]
    fn decrypt_test() {
        let result = decrypt("tfuwxjyabncefrgijvkmnzoqrd", "test");
        assert_eq!(result, "abcdefghijklmnopqrstuvwxyz");
    }
}
