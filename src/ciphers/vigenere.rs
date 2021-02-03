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
    /// let vigenere = Vigenere::new("ABCDE");
    ///
    /// assert_eq!("QXGUX", vigenere.encipher("QWERT"));
    /// ```
    pub fn encipher(&self, message: &str) -> String {
        let key = key_gen(self.key, message.len()).unwrap();

        Vigenere::vigenere_engine(message, &key, true)
    }

    /// Deciphers a cipher with the vigenere cipher.
    ///
    /// # Examples:
    /// ```
    /// use cienli::ciphers::vigenere::Vigenere;
    /// let vigenere = Vigenere::new("ABCDE");
    ///
    /// assert_eq!("QWERT", vigenere.decipher("QXGUX"));
    pub fn decipher(&self, message: &str) -> String {
        let key = key_gen(self.key, message.len()).unwrap();

        Vigenere::vigenere_engine(message, &key, false)
    }

    fn vigenere_engine(message: &str, key: &str, encipher: bool) -> String {
        let mut result: String = String::new();

        let key = key.as_bytes();
        let message = message.as_bytes();

        for indx in 0..message.len() {
            result.push(match encipher {
                true => (((key[indx] + message[indx]) % 26) + 65) as char,
                false => (((26 + message[indx] - key[indx]) % 26) + 65) as char,
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

        assert_eq!("QXGUX", v.encipher("QWERT"));
    }

    #[test]
    fn decipher_test() {
        let v = Vigenere::new("ABCDE");

        assert_eq!("QWERT", v.decipher("QXGUX"));
    }
}
