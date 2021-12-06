pub enum RotType {
    Rot5,
    Rot13,
    Rot18,
    Rot47,
}

/// Rot Cipher
///
/// The struct is generated through the new() function.
///
pub struct Rot<'a> {
    message: &'a str,
    rot_type: RotType,
}

impl Rot<'_> {
    /// Initialize a rot cipher with a message and rot type.
    ///
    /// # Examples:
    /// - Initialization with Rot13 type.:
    /// ```
    /// use cienli::ciphers::rot::{Rot, RotType};
    /// let rot = Rot::new("• Hello Friend 83110 :) •", RotType::Rot13);
    /// ```
    pub fn new(message: &str, rot_type: RotType) -> Rot {
        Rot { message, rot_type }
    }

    /// Enciphers a message with the rot cipher.
    ///
    /// # Examples:
    ///
    /// - Encipher with Rot47:
    /// ```
    /// use cienli::ciphers::rot::{Rot, RotType};
    /// let rot47 = Rot::new("• Hello Friend 83110 :) •", RotType::Rot47);
    ///
    /// assert_eq!("• w6==@ uC:6?5 gb``_ iX •", rot47.encipher());
    /// ```
    ///
    /// - Encipher with Rot13:
    /// ```
    /// use cienli::ciphers::rot::{Rot, RotType};
    /// let rot13 = Rot::new("• Hello Friend 83110 :) •", RotType::Rot13);
    ///
    /// assert_eq!("• Uryyb Sevraq 83110 :) •", rot13.encipher());
    /// ```
    pub fn encipher(&self) -> String {
        match self.rot_type {
            RotType::Rot5 => Rot::rot5(self.message),
            RotType::Rot13 => Rot::rot13(self.message),
            RotType::Rot18 => Rot::rot13(&(Rot::rot5(self.message))),
            RotType::Rot47 => Rot::rot47(self.message),
        }
    }

    /// Deciphers a cipher with the rot cipher.
    ///
    /// # Examples:
    ///
    /// - Decipher with Rot47:
    /// ```
    /// use cienli::ciphers::rot::{Rot, RotType};
    /// let rot47 = Rot::new("• w6==@ uC:6?5 gb``_ iX •", RotType::Rot47);
    ///
    /// assert_eq!("• Hello Friend 83110 :) •", rot47.decipher());
    /// ```
    ///
    /// - Decipher with Rot13:
    /// ```
    /// use cienli::ciphers::rot::{Rot, RotType};
    /// let rot13 = Rot::new("• Uryyb Sevraq 83110 :) •", RotType::Rot13);
    ///
    /// assert_eq!("• Hello Friend 83110 :) •", rot13.decipher());
    /// ```
    pub fn decipher(&self) -> String {
        self.encipher()
    }

    fn rot5(message: &str) -> String {
        message
            .chars()
            .map(|digit| match digit {
                '0'..='4' => ((digit as u8) + 5) as char,
                '5'..='9' => ((digit as u8) - 5) as char,
                _ => digit,
            })
            .collect()
    }

    fn rot13(message: &str) -> String {
        message
            .chars()
            .map(|character| match character {
                'A'..='M' | 'a'..='m' => ((character as u8) + 13) as char,
                'N'..='Z' | 'n'..='z' => ((character as u8) - 13) as char,
                _ => character,
            })
            .collect()
    }

    fn rot47(message: &str) -> String {
        message
            .chars()
            .map(|character| match character {
                '!'..='O' => ((character as u8) + 47) as char,
                'P'..='~' => ((character as u8) - 47) as char,
                _ => character,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Rot, RotType};

    #[test]
    fn rot47_encipher() {
        let rot47 = Rot::new("• Hello Friend 83110 :) •", RotType::Rot47);

        assert_eq!("• w6==@ uC:6?5 gb``_ iX •", rot47.encipher());
    }

    #[test]
    fn rot47_decipher() {
        let rot47 = Rot::new("• w6==@ uC:6?5 gb``_ iX •", RotType::Rot47);

        assert_eq!("• Hello Friend 83110 :) •", rot47.decipher());
    }

    #[test]
    fn rot18_encipher() {
        let rot18 = Rot::new("• Hello Friend 83110 :) •", RotType::Rot18);

        assert_eq!("• Uryyb Sevraq 38665 :) •", rot18.encipher());
    }

    #[test]
    fn rot18_decipher() {
        let rot18 = Rot::new("• Uryyb Sevraq 38665 :) •", RotType::Rot18);

        assert_eq!("• Hello Friend 83110 :) •", rot18.decipher());
    }

    #[test]
    fn rot13_encipher() {
        let rot13 = Rot::new("• Hello Friend 83110 :) •", RotType::Rot13);

        assert_eq!("• Uryyb Sevraq 83110 :) •", rot13.encipher());
    }

    #[test]
    fn rot13_decipher() {
        let rot13 = Rot::new("• Uryyb Sevraq 83110 :) •", RotType::Rot13);

        assert_eq!("• Hello Friend 83110 :) •", rot13.decipher());
    }

    #[test]
    fn rot5_encipher() {
        let rot5 = Rot::new("• Hello Friend 83110 :) •", RotType::Rot5);

        assert_eq!("• Hello Friend 38665 :) •", rot5.encipher());
    }

    #[test]
    fn rot5_decipher() {
        let rot5 = Rot::new("• Hello Friend 38665 :) •", RotType::Rot5);

        assert_eq!("• Hello Friend 83110 :) •", rot5.decipher());
    }
}
