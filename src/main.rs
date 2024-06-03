use std::fmt::{Display, Formatter, Result};

struct RLPEncoding {
    encoding: Vec<u8>,
}

impl Display for RLPEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Convert each byte to its hexadecimal representation and join them
        let hex_string: String = self
            .encoding
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "0x{}", hex_string)
    }
}

impl From<&str> for RLPEncoding {
    fn from(value: &str) -> Self {
        let mut encoding: Vec<u8> = vec![];

        if value.is_empty() {
            encoding = Vec::from([0x80]);
        } else if value.len() == 1 && (0x00..0x7f).contains(&value.as_bytes()[0]) {
            encoding = Vec::from([value.as_bytes()[0]])
        }
        Self { encoding }
    }
}

impl<T> From<Vec<T>> for RLPEncoding {
    fn from(list: Vec<T>) -> Self {
        let mut encoding: Vec<u8> = vec![];

        if list.is_empty() {
            encoding = Vec::from([0xc0]);
        }

        Self { encoding }
    }
}

fn main() {
    //let input: Vec<u8> = vec![];
    //let input = "";
    let input = "a";
    //let input = "test";
    println!("[rlp-encoding] starting to encode");

    let out = RLPEncoding::from(input);
    println!("[rlp-encoding] final encoding: {}", out);
}
