use key::key;

// key!() generate a random number from range 5..50
// the same problem with using const
pub static KEY: u8 = key!();

pub fn encode(key: u8, value: &str) -> String {
    let value_bytes = value.as_bytes();

    let mut output = Vec::new();

    for byte in value_bytes {
        let byte = *byte as usize;

        let byte_out = byte << key;

        output.push(byte_out.to_string());
    }

    output.join(" ")
}

pub fn decode(value: &str) -> String {
    decode_key(KEY, value)
}

pub fn decode_key(key: u8, value: &str) -> String {
    let mut output = Vec::new();

    let bytes: Vec<&str> = value.split_ascii_whitespace().collect();

    for byte in bytes {
        let byte: usize = byte.parse().unwrap();

        let out = byte >> key;

        output.push(out as u8);
    }

    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(key: u8) {
        let value = "1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM~!@#$%^&*()_+-={}|[]\\:;\"',./<>?";

        let output_enc = encode(key, value);

        let output_dec = decode_key(key, &output_enc);

        assert_eq!(output_dec, value)
    }

    #[test]
    fn test() {
        for i in 1..50 {
            check(i);
        }
    }
}
