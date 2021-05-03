pub struct Scytale {
    key: usize,
}

impl Scytale {
    pub fn new(key: usize) -> Result<Scytale, &'static str> {
        match key {
            0 => Err("Key cannot be zero"),
            _ => Ok(Scytale { key: key }),
        }
    }

    pub fn encipher(&self, message: &str) -> String {
        if self.key >= message.chars().count() {
            return message.to_string();
        }

        let table = Scytale::generate_table(self.key, message, false);

        table
            .iter()
            .flatten()
            .collect::<String>()
            .trim_end_matches("\0")
            .to_string()
    }

    pub fn decipher(&self, cipher: &str) -> String {
        if self.key >= cipher.chars().count() || self.key == 1 {
            return cipher.to_string();
        }

        let mut table = Scytale::generate_table(self.key, cipher, true);

        let mut message = String::new();
        while table
            .iter()
            .filter(|character| !character.is_empty())
            .count()
            > 0
        {
            for column in table.iter_mut() {
                message.push(column.remove(0));
            }
        }
        message.trim_end_matches("\0").to_string()
    }

    fn generate_table(height: usize, message: &str, decipher: bool) -> Vec<Vec<char>> {
        let width = (message.chars().count() as f32 / height as f32).ceil() as usize;

        let mut table = vec![vec!['\0'; width]; height];

        for (position, element) in message.chars().enumerate() {
            let (column, row) = match decipher {
                true => (position / height, position % height),
                false => (position % height, position / height),
            };

            table[column][row] = element;
        }

        table
    }
}

#[cfg(test)]
mod tests {
    use super::Scytale;

    #[test]
    fn invalid_key_test() {
        assert!(Scytale::new(0).is_err());
    }

    #[test]
    fn big_key_test() {
        let scytale = Scytale::new(15).unwrap();
        assert_eq!("Hello :)", scytale.encipher("Hello :)"))
    }

    #[test]
    fn equal_key_test() {
        let scytale = Scytale::new(8).unwrap();
        assert_eq!("Hello :)", scytale.encipher("Hello :)"))
    }

    #[test]
    fn encipher_test() {
        let scytale = Scytale::new(3).unwrap();
        assert_eq!("Hl:eo)l ", scytale.encipher("Hello :)"));
    }

    #[test]
    fn decipher_test() {
        let scytale = Scytale::new(3).unwrap();
        assert_eq!("Hello :)", scytale.decipher("Hl:eo)l "));
    }
}
