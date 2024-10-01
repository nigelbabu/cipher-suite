pub mod vignere {
    use std::string::String;
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

    pub fn encrypt(input: &str, key: &str) -> Result<String, String> {
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
        match String::from_utf8(result) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e))
        }
    }

    pub fn decrypt(input: &str, key: &str) -> Result<String, String> {
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
        match String::from_utf8(result) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e))
        }
    }
}

pub mod caesar {
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

    pub fn encrypt(input: &str, shift: u8) -> Result<String, String> {
        let mut result: Vec<u8> = Vec::new();
        for letter in input.chars() {
            result.push(match letter {
                l @ 'a'..='z' => shift_right(l as u8, shift, 'a' as u8),
                l @ 'A'..='Z' => shift_right(l as u8, shift, 'A' as u8),
                l @ _ => l as u8,
            });
        }
        match String::from_utf8(result) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e))
        }
    }

    pub fn decrypt(input: &str, shift: u8) -> Result<String, String> {
        let mut result: Vec<u8> = Vec::new();
        for letter in input.chars() {
            result.push(match letter {
                l @ 'a'..='z' => shift_left(l as u8, shift, 'a' as u8),
                l @ 'A'..='Z' => shift_left(l as u8, shift, 'A' as u8),
                l @ _ => l as u8,
            });
        }
        match String::from_utf8(result) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_simple_vignere_test() {
        let result = vignere::encrypt("This is a test", "test");
        assert_eq!(result.unwrap(), "Mlal bw s mxwl");
    }
    #[test]
    fn decrypt_simple_vignere_test() {
        let result = vignere::decrypt("Mlal bw s mxwl", "test");
        assert_eq!(result.unwrap(), "This is a test");
    }
    #[test]
    fn encrypt_vignere_test() {
        let result = vignere::encrypt("abcdefghijklmnopqrstuvwxyz", "test");
        assert_eq!(result.unwrap(), "tfuwxjyabncefrgijvkmnzoqrd");
    }
    #[test]
    fn decrypt_vignere_test() {
        let result = vignere::decrypt("tfuwxjyabncefrgijvkmnzoqrd", "test");
        assert_eq!(result.unwrap(), "abcdefghijklmnopqrstuvwxyz");
    }
    #[test]
    fn encrypt_simple_caesar_test() {
        let result = caesar::encrypt("This is a test", 3);
        assert_eq!(result.unwrap(), "Wklv lv d whvw");
    }
    #[test]
    fn decrypt_simple_caesar_test() {
        let result = caesar::decrypt("Wklv lv d whvw", 3);
        assert_eq!(result.unwrap(), "This is a test");
    }
    #[test]
    fn encrypt_caesar_test() {
        let result = caesar::encrypt("abcdefghijklmnopqrstuvwxyz", 3);
        assert_eq!(result.unwrap(), "defghijklmnopqrstuvwxyzabc");
    }
    #[test]
    fn decrypt_caesar_test() {
        let result = caesar::decrypt("abcdefghijklmnopqrstuvwxyz", 3);
        assert_eq!(result.unwrap(), "xyzabcdefghijklmnopqrstuvw");
    }
}

