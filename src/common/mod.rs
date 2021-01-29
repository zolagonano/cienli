pub fn key_gen(key: &str, message_len: usize) -> Result<String, &'static str> {
    let mut result: String = String::from(key);

    if key.len() <= 0 || message_len <= 0 {
        return Err("Error: Key and Message length must be 1 or greater than 1!!");
    } else {
        if key.len() == message_len {
            return Ok(key.to_string());
        } else {
            if key.len() > message_len {
                return Ok(key[..message_len].to_string());
            } else {
                if key.len() < message_len {
                    for left in 0..(message_len - key.len()) {
                        result.push(key.as_bytes()[left % key.len()] as char);
                    }
                }
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::key_gen;

    #[test]
    fn make_key_bigger() {
        assert_eq!(String::from("TESTTESTTE"), key_gen("TEST", 10).unwrap());
    }

    #[test]
    fn make_key_smaller() {
        assert_eq!(String::from("TE"), key_gen("TEST", 2).unwrap());
    }

    #[test]
    fn key_is_equal() {
        assert_eq!(String::from("TEST"), key_gen("TEST", 4).unwrap());
    }

    #[test]
    fn too_small_value() {
        assert!(key_gen("TEST", 0).is_err());

        assert!(key_gen("", 4).is_err());
    }
}
