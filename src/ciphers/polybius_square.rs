const TABLE: [[char; 5]; 5] = [
    ['A', 'B', 'C', 'D', 'E'],
    ['F', 'G', 'H', 'J', 'K'],
    ['L', 'M', 'N', 'O', 'P'],
    ['Q', 'R', 'S', 'T', 'U'],
    ['V', 'W', 'X', 'Y', 'Z'],
]; // Removed letter: I
pub struct PolybiusSquare<'a> {
    message: &'a str,
}

impl PolybiusSquare<'_> {
    pub fn new(message: &str) -> PolybiusSquare {
        PolybiusSquare { message: message }
    }

    pub fn encipher(&self) -> String {
        self.message
            .chars()
            .map(|character| match character {
                'A'..='Z' => PolybiusSquare::encipher_engine(65, 'K', 'J', character),
                'a'..='z' => PolybiusSquare::encipher_engine(97, 'k', 'j', character),
                _ => String::from(""),
            })
            .collect()
    }

    pub fn decipher(&self) -> Result<String, &'static str> {
        if self.message.len() % 2 != 0 {
            return Err("1 column is missing");
        }
        if !PolybiusSquare::is_string_numeric(self.message) {
            return Err("Ciphertext must be numeric");
        }

        let mut result = String::new();

        let cipher_len = self.message.len();
        for i in 1..(cipher_len / 2) + 1 {
            let row_and_col = &self.message.as_bytes()[(i * 2) - 2..i * 2];
            let row: usize = (row_and_col[0] as char)
                .to_string()
                .parse::<usize>()
                .unwrap()
                - 1;
            let col: usize = (row_and_col[1] as char)
                .to_string()
                .parse::<usize>()
                .unwrap()
                - 1;
            result.push(TABLE[row][col]);
        }

        Ok(result)
    }

    fn encipher_engine(a: u8, k: char, j: char, character: char) -> String {
        let mut row = ((character as u8 - a) / 5) + 1;
        let mut col = ((character as u8 - a) % 5) + 1;

        if character == k {
            row = row - 1;
            col = 5 - col + 1;
        } else if character >= j {
            if col == 1 {
                col = 6;
                row = row - 1;
            }
            col = col - 1;
        }

        format!("{}{}", row, col)
    }

    fn is_string_numeric(text: &str) -> bool {
        for character in text.chars() {
            if !character.is_numeric() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::PolybiusSquare;

    #[test]
    fn encipher_test() {
        let polybius = PolybiusSquare::new("Hello World :)");

        assert_eq!("23153131345234423114", polybius.encipher());
    }

    #[test]
    fn decipher_test() {
        let polybius = PolybiusSquare::new("23153131345234423114");

        assert_eq!("HELLOWORLD", polybius.decipher().unwrap());
    }
}
