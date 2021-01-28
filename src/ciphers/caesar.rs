pub struct Caesar {
    rotation: u8,
}

impl Caesar {
    pub fn new(rotation: u8) -> Result<Caesar, &'static str> {
        if rotation >= 1 && rotation <= 26 {
            Ok(Caesar { rotation: rotation })
        } else {
            Err("Error: Rotation must be in range 1 and 26!!")
        }
    }

    pub fn encipher(&self, message: &str) -> String {
        Caesar::shift(message, self.rotation)
    }

    pub fn decipher(&self, message: &str) -> String {
        Caesar::shift(message, 26 - self.rotation)
    }

    fn shift(message: &str, rotation: u8) -> String {
        message
            .chars()
            .map(|character| match character {
                'A'..='Z' => (((character as u8 - 65 + rotation) % 26) + 65) as char,
                'a'..='z' => (((character as u8 - 97 + rotation) % 26) + 97) as char,
                _ => character,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Caesar;

    #[test]
    fn encipher_test() {
        let caesar = Caesar::new(5).unwrap();

        assert_eq!(
            String::from("Mjqqt, Ymnx Nx F Yjxy"),
            caesar.encipher("Hello, This Is A Test")
        );
    }

    #[test]
    fn decipher_test() {
        let caesar = Caesar::new(5).unwrap();

        assert_eq!(
            String::from("Hello, This Is A Test"),
            caesar.decipher("Mjqqt, Ymnx Nx F Yjxy")
        );
    }

    #[test]
    fn big_rotation() {
        assert!(Caesar::new(34).is_err());
    }

    #[test]
    fn small_rotation() {
        assert!(Caesar::new(0).is_err());
    }
}
