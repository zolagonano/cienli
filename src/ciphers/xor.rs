use crate::common::key_gen;

/// Xor Cipher
///
/// The struct is generated through the new() function.
///
pub struct Xor<'a> {
    key: &'a str,
}

impl Xor<'_> {
    /// Initialize a xor cipher with the key.
    ///
    /// # Examples:
    /// ```
    /// use cienli::ciphers::xor::Xor;
    /// let xor = Xor::new("VMMN8");
    /// ```
    pub fn new(key: &str) -> Xor {
        Xor { key }
    }

    /// Enciphers a message with the xor cipher.
    ///
    /// # Examples:
    /// ```
    /// use cienli::ciphers::xor::Xor;
    /// let xor = Xor::new("VMMN8");
    ///
    /// assert_eq!("<=|zv", xor.encipher("jp14N"));
    /// ```
    pub fn encipher(&self, message: &str) -> String {
        let key = key_gen(self.key, message.len()).unwrap();

        Xor::xor_engine(message, &key)
    }

    /// Deciphers a cipher with the xor cipher.
    ///
    /// # Examples:
    /// ```
    /// use cienli::ciphers::xor::Xor;
    /// let xor = Xor::new("VMMN8");
    /// assert_eq!("jp14N", xor.decipher("<=|zv"));
    /// ```
    pub fn decipher(&self, cipher: &str) -> String {
        let key = key_gen(self.key, cipher.len()).unwrap();

        Xor::xor_engine(cipher, &key)
    }

    fn xor_engine(message: &str, key: &str) -> String {
        let mut result: String = String::new();

        let message = message.as_bytes();
        let key = key.as_bytes();

        for indx in 0..message.len() {
            result.push((message[indx] ^ key[indx]) as char)
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Xor;

    #[test]
    fn encipher() {
        let xor = Xor::new("VMMN8");

        assert_eq!("<=|zv", xor.encipher("jp14N"));
    }

    #[test]
    fn decipher() {
        let xor = Xor::new("VMMN8");

        assert_eq!("jp14N", xor.decipher("<=|zv"));
    }
}
