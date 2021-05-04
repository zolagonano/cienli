use crate::common::key_gen;

/// Vigenere Cipher
///
/// The struct is generated through the new() function.
///
pub struct Vigenere<'a> {
    key: &'a str,
}

impl Vigenere<'_> {
    /// Initialize a vignere cipher with a key.
    ///
    /// # Examples:
    /// ```
    /// use cienli::ciphers::vigenere::Vigenere;
    /// let vigenere = Vigenere::new("ABCDE");
    /// ```
    pub fn new(key: &str) -> Vigenere {
        Vigenere { key: key }
    }

    /// Enciphers a message with the vigenere cipher.
    ///
    /// # Examples:
    /// ```
    /// use cienli::ciphers::vigenere::Vigenere;
    /// let vigenere = Vigenere::new("ABcdE");
    ///
    /// assert_eq!("Qxgux :)", vigenere.encipher("Qwert :)"));
    /// ```
    pub fn encipher(&self, message: &str) -> String {
        let key = key_gen(&self.key.to_uppercase(), message.len())
            .unwrap()
            .as_bytes()
            .to_owned();

        let message = message.as_bytes();

        let mut result: String = String::new();

        for indx in 0..message.len() {
            result.push(match message[indx] as char {
                'A'..='Z' => (((key[indx] + message[indx]) % 26) + 65) as char,
                'a'..='z' => ((((key[indx] - 32) + message[indx]) % 26) + 97) as char,
                _ => message[indx] as char,
            });
        }
        result
    }

    /// Deciphers a cipher with the vigenere cipher.
    ///
    /// # Examples:
    /// ```
    /// use cienli::ciphers::vigenere::Vigenere;
    /// let vigenere = Vigenere::new("ABcdE");
    ///
    /// assert_eq!("Qwert :)", vigenere.decipher("Qxgux :)"));
    pub fn decipher(&self, message: &str) -> String {
        let key = key_gen(&self.key.to_uppercase(), message.len())
            .unwrap()
            .as_bytes()
            .to_owned();

        let message = message.as_bytes();

        let mut result: String = String::new();

        for indx in 0..message.len() {
            result.push(match message[indx] as char {
                'A'..='Z' => (((26 + message[indx] - key[indx]) % 26) + 65) as char,
                'a'..='z' => (((26 + message[indx] - (key[indx] + 32)) % 26) + 97) as char,
                _ => message[indx] as char,
            });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Vigenere;

    #[test]
    fn encipher_test() {
        let v = Vigenere::new("ABCDE");

        assert_eq!("QXGUX :)", v.encipher("QWERT :)"));
    }

    #[test]
    fn decipher_test() {
        let v = Vigenere::new("ABCDE");

        assert_eq!("Qwert :)", v.decipher("Qxgux :)"));
    }
}
