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
        } else if (2..55).contains(&value.len()) {
            let encoding_byte = 0x80 + value.len();
            encoding = Vec::from([encoding_byte as u8]);
            encoding.extend_from_slice(value.as_bytes());
        } else {
            let mut b_len = Vec::new();
            let mut x = value.len();
            while x > 0 {
                b_len.push((x % 256) as u8);
                x /= 256;
            }
            b_len.reverse();
            let encoding_byte = 0xb7 + b_len.len();
            encoding = Vec::from([encoding_byte as u8]);
            encoding.extend_from_slice(&b_len);
            encoding.extend_from_slice(value.as_bytes());
        }
        Self { encoding }
    }
}

impl<T: Into<RLPEncoding>> From<Vec<T>> for RLPEncoding {
    fn from(list: Vec<T>) -> Self {
        let mut encoding: Vec<u8> = vec![];

        if list.is_empty() {
            encoding = Vec::from([0xc0]);
        }

        let mut concat: Vec<u8> = vec![];
        for item in list {
            concat.extend_from_slice(&(item.into()).encoding);
        }

        let encoding_byte = 0xc0 + concat.len();
        if concat.len() <= 55 {
            encoding.extend_from_slice(&[encoding_byte as u8]);
            encoding.extend_from_slice(&concat);
        } else {
            let mut b_len = Vec::new();
            let mut x = concat.len();
            while x > 0 {
                b_len.push((x % 256) as u8);
                x /= 256;
            }
            b_len.reverse();
            let encoding_byte = 0xf7 + b_len.len();
            encoding = Vec::from([encoding_byte as u8]);
            encoding.extend_from_slice(&b_len);
            encoding.extend_from_slice(&concat);
        }

        Self { encoding }
    }
}

fn main() {
    //let input: Vec<u8> = vec![];
    //let input = "";
    //let input = "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    //let input = "Bonjour les amis comment ca va, moi ca va super et je suis entrain d'ecrire ca car c'est vraiment super rigolo d'ecrire. Recommencons. Bonjour les amis comment ca va, moi ca va super et je suis entrain d'ecrire ca car c'est vraiment super rigolo d'ecrire lol.";
    //let input = vec!["bonjour", "les", "amis", "ca", "va", "?"];
    //let input = vec!["cat", "dog"];
    let input = vec![
        "apple", "bread", "cheese", "date", "egg", "fig", "guava", "honey", "ice", "jam", "kale",
    ];
    println!("[rlp-encoding] starting to encode");

    let out = RLPEncoding::from(input);
    println!("[rlp-encoding] final encoding: {}", out);
}
