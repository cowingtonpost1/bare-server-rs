const VALID_CHARS: &str =
    "!#$%&'*+-.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ^_`abcdefghijklmnopqrstuvwxyz|~";
const RESERVED_CHARS: &str = "%";

pub fn is_valid_protocol(input: &str) -> bool {
    for c in input.chars() {
        if !VALID_CHARS.contains(c) {
            return false;
        }
    }
    return true;
}

pub fn encode_protocol(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if VALID_CHARS.contains(c) && !RESERVED_CHARS.contains(c) {
            result.push(c);
        } else {
            result.push_str(format!("%{:X}", c as u32).as_str())
        }
    }

    return result;
}

pub fn decode_protocol(input: &str) -> Option<String> {
    let mut result = String::new();
    let mut skip = 0;
    for (i, c) in input.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        if c == '%' {
            let code = u32::from_str_radix(&input[i + 1..i + 3], 16).ok()?;
            result.push(char::from_u32(code)?);
            skip += 2;
        } else {
            result.push(c);
        }
    }

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_protocol() {
        assert_eq!(encode_protocol("1/100%").as_str(), "1%2F100%25");
    }

    #[test]
    fn test_decode_protocol() {
        assert_eq!(decode_protocol("1%2F100%25").unwrap().as_str(), "1/100%");
    }

    #[test]
    fn test_is_valid_protocol() {
        assert!(is_valid_protocol("1%2F100%25"))
    }
}
