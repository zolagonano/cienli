pub struct Atbash<'a> {
    message: &'a str,
}

impl Atbash<'_> {
    pub fn new(message: &str) -> Atbash {
        Atbash { message: message }
    }

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
