/// Atbash Cipher
///
/// The struct is generated through the new() function
///
pub struct Atbash<'a> {
    message: &'a str,
}

impl Atbash<'_> {
    /// Initialize a atbash cipher with a message or a cipher.
    ///
    /// # Examples:
    /// - Initialization with a message:
    /// ```
    /// use cienli::ciphers::atbash::Atbash;
    /// let atbash = Atbash::new("Hello Friend :)");
    /// ```
    ///
    /// - Initialization with a cipher:
    /// ```
    /// use cienli::ciphers::atbash::Atbash;
    /// let atbash = Atbash::new("Svool Uirvmw :)");
    /// ```
    ///
    pub fn new(message: &str) -> Atbash {
        Atbash { message: message }
    }

    /// Enciphers a message with the atbash cipher.
    ///
    /// # Example:
    /// ```
    /// use cienli::ciphers::atbash::Atbash;
    /// let atbash = Atbash::new("Hello Friend :)");
    ///
    /// assert_eq!("Svool Uirvmw :)", atbash.encipher());
    /// ```
    pub fn encipher(&self) -> String {
        self.message
            .chars()
            .map(|character| match character {
                'A'..='Z' => ((90 - character as u8) + 65) as char,
                'a'..='z' => ((122 - character as u8) + 97) as char,
                _ => character,
            })
            .collect()
    }

    /// Deciphers a message with the atbash cipher.
    ///
    /// # Example:
    /// ```
    /// use cienli::ciphers::atbash::Atbash;
    /// let atbash = Atbash::new("Svool Uirvmw :)");
    ///
    /// assert_eq!("Hello Friend :)", atbash.decipher());
    /// ```
    pub fn decipher(&self) -> String {
        self.encipher()
    }
}

#[cfg(test)]
mod tests {
    use super::Atbash;

    #[test]
    fn atbash_encipher() {
        let atbash = Atbash::new("Hello Friend :)");
        assert_eq!("Svool Uirvmw :)", atbash.encipher())
    }

    #[test]
    fn atbash_decipher() {
        let atbash = Atbash::new("Svool Uirvmw :)");
        assert_eq!("Hello Friend :)", atbash.decipher());
    }
}
