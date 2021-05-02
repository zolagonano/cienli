use num_integer::Integer;

pub struct Affine {
    alpha: u16,
    beta: u16,
}

impl Affine {
    pub fn new(key: (u16, u16)) -> Result<Affine, &'static str> {
        let is_key_valid = Affine::key_checker(key);

        match is_key_valid {
            true => Ok(Affine {
                alpha: key.0,
                beta: key.1,
            }),
            false => Err("there is a problem with the key"),
        }
    }

    pub fn encipher(&self, message: &str) -> String {
        message
            .chars()
            .map(|character| match character {
                'a'..='z' => {
                    (((character as u16 - 97) * self.alpha + self.beta) % 26 + 97) as u8 as char
                }
                'A'..='Z' => {
                    (((character as u16 - 65) * self.alpha + self.beta) % 26 + 65) as u8 as char
                }
                _ => character,
            })
            .collect()
    }

    pub fn decipher(&self, message: &str) -> String {
        let mut alpha_inv = 0;
        while (self.alpha * alpha_inv) % 26 != 1 {
            alpha_inv += 1;
        }

        message
            .chars()
            .map(|character| match character {
                'a'..='z' => {
                    (alpha_inv * ((character as u16 - 97) - self.beta) % 26 + 97) as u8 as char
                }
                'A'..='Z' => {
                    (alpha_inv * ((character as u16 - 65) - self.beta) % 26 + 65) as u8 as char
                }
                _ => character,
            })
            .collect()
    }

    fn key_checker(key: (u16, u16)) -> bool {
        if (key.0 >= 1 && key.0 <= 26) && key.1 <= 26 {
            if key.0.gcd(&26) == 1 {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Affine;

    #[test]
    fn invalid_key_length_test() {
        assert!(Affine::new((27, 2)).is_err())
    }

    #[test]
    fn invalid_key_coprime_test() {
        assert!(Affine::new((10, 2)).is_err())
    }

    #[test]
    fn encipher_test() {
        let affine = Affine::new((5, 2)).unwrap();
        assert_eq!("Lwffu :)", affine.encipher("Hello :)"))
    }

    #[test]
    fn decipher_test() {
        let affine = Affine::new((5, 2)).unwrap();
        assert_eq!("Hello :)", affine.decipher("Lwffu :)"))
    }
}
