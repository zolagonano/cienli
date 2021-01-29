use regex::{Captures, Regex};

pub struct Bacon {
    letters: (char, char),
}

impl Bacon {
    pub fn new(letters: (char, char)) -> Result<Bacon, &'static str> {
        if letters.0 == letters.1 {
            return Err("Error: Letters must be different from each other!!");
        }
        Ok(Bacon { letters: letters })
    }

    pub fn encipher(&self, message: &str) -> String {
        message
            .to_ascii_uppercase()
            .chars()
            .map(|character| match character {
                'A'..='Z' => format!("{:05b}", character as usize - 65)
                    .replace("0", &self.letters.0.to_string())
                    .replace("1", &self.letters.1.to_string()),
                _ => character.to_string(),
            })
            .collect::<String>()
    }

    pub fn decipher(&self, message: &str) -> String {
        let binary_message = message
            .replace(&self.letters.0.to_string(), "0")
            .replace(&self.letters.1.to_string(), "1");

        let re = Regex::new(r"[01]{5}").unwrap();

        let result = re.replace_all(&binary_message, |cap: &Captures| {
            ((u8::from_str_radix(&cap[0], 2).unwrap() + 65) as char).to_string()
        });

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Bacon;

    #[test]
    fn encipher_test() {
        let bacon = Bacon::new(('a', 'b')).unwrap();

        assert_eq!(
            String::from("aabbbaabaaababbababbabbba aababbaaababaaaaabaaabbabaaabb"),
            bacon.encipher("Hello Friend")
        );
    }

    #[test]
    fn decipher_test() {
        let bacon = Bacon::new(('a', 'b')).unwrap();

        assert_eq!(
            String::from("HELLO FRIEND"),
            bacon.decipher("aabbbaabaaababbababbabbba aababbaaababaaaaabaaabbabaaabb")
        );
    }

    #[test]
    fn encipher_with_different_letters() {
        let bacon = Bacon::new(('+', '=')).unwrap();

        assert_eq!(
            String::from("++===++=+++=+==+=+==+===+ ++=+==+++=+=+++++=+++==+=+++=="),
            bacon.encipher("Hello Friend")
        );
    }

    #[test]
    fn decipher_with_different_letters() {
        let bacon = Bacon::new(('+', '=')).unwrap();

        assert_eq!(
            String::from("HELLO FRIEND"),
            bacon.decipher("++===++=+++=+==+=+==+===+ ++=+==+++=+=+++++=+++==+=+++==")
        );
    }

    #[test]
    fn same_letters() {
        assert!(Bacon::new(('a', 'a')).is_err());
    }
}
