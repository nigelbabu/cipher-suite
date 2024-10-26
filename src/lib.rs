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
                    let res = shift_right(l as u8, (key[i % key.len()]) - b'a', b'a');
                    i += 1;
                    res
                }
                l @ 'A'..='Z' => {
                    let res = shift_right(l as u8, (key[i % key.len()]) - b'a', b'A');
                    i += 1;
                    res
                }
                l @ _ => l as u8,
            });
        }
        match String::from_utf8(result) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e)),
        }
    }

    pub fn decrypt(input: &str, key: &str) -> Result<String, String> {
        let mut result: Vec<u8> = Vec::new();
        let key = key.as_bytes();
        let mut i = 0;
        for letter in input.chars() {
            result.push(match letter {
                l @ 'a'..='z' => {
                    let res = shift_left(l as u8, (key[i % key.len()]) - b'a', b'a');
                    i += 1;
                    res
                }
                l @ 'A'..='Z' => {
                    let res = shift_left(l as u8, (key[i % key.len()]) - b'a', b'A');
                    i += 1;
                    res
                }
                l @ _ => l as u8,
            });
        }
        match String::from_utf8(result) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e)),
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
                l @ 'a'..='z' => shift_right(l as u8, shift, b'a'),
                l @ 'A'..='Z' => shift_right(l as u8, shift, b'A'),
                l @ _ => l as u8,
            });
        }
        match String::from_utf8(result) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e)),
        }
    }

    pub fn decrypt(input: &str, shift: u8) -> Result<String, String> {
        let mut result: Vec<u8> = Vec::new();
        for letter in input.chars() {
            result.push(match letter {
                l @ 'a'..='z' => shift_left(l as u8, shift, b'a'),
                l @ 'A'..='Z' => shift_left(l as u8, shift, b'A'),
                l @ _ => l as u8,
            });
        }
        match String::from_utf8(result) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e)),
        }
    }
}

pub mod hill {

    pub fn encrypt(input: &str, key: &str) -> Result<String, String> {
        let mut key_matrix = vec![vec![0; 2]; 2];
        let mut chars = key.chars();
        for i in 0..2 {
            for j in 0..2 {
                key_matrix[i][j] = match chars.next() {
                    Some(x) => (x as u8) % (b'a'),
                    None => 0,
                }
            }
        }
        let mut input_matrix: Vec<Vec<u8>> = Vec::new();
        let mut y = 0;

        let mut item = vec![25; 2];
        for (i, letter) in input.chars().enumerate() {
            match letter {
                l @ 'a'..='z' => {
                    item[y] = (l as u8) % (b'a');
                    y += 1;
                }
                l @ 'A'..='Z' => {
                    item[y] = (l as u8) % (b'A');
                    y += 1;
                }
                _ => (),
            }
            if y > 1 || (y != 0 && i == input.len() - 1) {
                y = 0;
                input_matrix.push(item.clone());
                item = vec![25; 2];
            }
        }

        let mut result: Vec<u8> = Vec::new();
        for item in input_matrix {
            for i in 0..2 {
                let mut res: u16 = 0;
                for j in 0..2 {
                    res += (item[j] as u16) * (key_matrix[i][j] as u16);
                }
                result.push((res % 26) as u8);
            }
        }

        let mut output: Vec<u8> = Vec::new();
        let mut i = 0;
        for letter in input.chars() {
            output.push(match letter {
                'a'..='z' => {
                    let r = result[i] + (b'a');
                    i += 1;
                    r
                }
                'A'..='Z' => {
                    let r = result[i] + (b'A');
                    i += 1;
                    r
                }
                l @ _ => l as u8,
            });
        }
        match String::from_utf8(output) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("Invalid UTF-8 sequence: {}", e)),
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
    #[test]
    fn encrypt_simple_hill_test() {
        let result = hill::encrypt("this is a test", "test");
        assert_eq!(result.unwrap(), "zhqs qs y xsyt");
    }
    #[test]
    fn decrypt_simple_hill_test() {
        let result = hill::encrypt("zhqs qs y xsytL", "pqup");
        assert_eq!(result.unwrap(), "this is a testZ");
    }
    #[test]
    fn encrypt_hill_test() {
        let result = hill::encrypt("abcdefghijklmnopqrstuvwxyz", "test");
        assert_eq!(result.unwrap(), "etypslmhgdazuvorincjwfqbkx");
    }
    #[test]
    fn decrypt_hill_test() {
        let result = hill::encrypt("etypslmhgdazuvorincjwfqbkx", "pqup");
        assert_eq!(result.unwrap(), "abcdefghijklmnopqrstuvwxyz");
    }
}
