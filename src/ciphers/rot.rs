pub enum RotType{
    Rot5,
    Rot13,
    Rot18,
    Rot47,
}

pub struct Rot<'a>{
    message: &'a str,
    rot_type: RotType,
}

impl Rot<'_>{
    pub fn new(message: &str, rot_type: RotType) -> Rot{
        Rot{
            message: message,
            rot_type: rot_type,
        }
    }

    pub fn encipher(&self) -> String{
        match self.rot_type{
            RotType::Rot5 => return Rot::rot5(self.message),
            RotType::Rot13 => return Rot::rot13(self.message),
            RotType::Rot18 => return Rot::rot13(&(Rot::rot5(self.message))),
            RotType::Rot47 => return Rot::rot47(self.message),
        }
    }

    pub fn decipher(&self) -> String{
        self.encipher()
    }

    fn rot5(message: &str) -> String{
        message
            .chars()
            .map(|digit| match digit{
                '0'..='4' => ((digit as u8) + 5) as char,
                '5'..='9' => ((digit as u8) - 5) as char,
                _ => digit,
            }).collect()
    }

    fn rot13(message: &str) -> String{
        message
            .chars()
            .map(|character| match character{
                'A'..='M' | 'a'..='m' => ((character as u8) + 13) as char,
                'N'..='Z' | 'n'..='z' => ((character as u8) - 13) as char,
                _ => character,
            }).collect()
    }

    fn rot47(message: &str) -> String{
        message
            .chars()
            .map(|character| match character{
                '!'..='O' => ((character as u8) + 47) as char,
                'P'..='~' => ((character as u8) - 47) as char,
                _ => character,
            }).collect()
    }
}

#[cfg(test)]
mod tests{
    use super::{Rot, RotType};

    #[test]
    fn rot47_encipher() {
        let rot47 = Rot::new("• Hello Friend 83110 :) •", RotType::Rot47);

        assert_eq!(String::from("• w6==@ uC:6?5 gb``_ iX •"), rot47.encipher());
    }

    #[test]
    fn rot47_decipher() {
        let rot47 = Rot::new("• w6==@ uC:6?5 gb``_ iX •", RotType::Rot47);

        assert_eq!(String::from("• Hello Friend 83110 :) •"), rot47.decipher());
    }


    #[test]
    fn rot18_encipher() {
        let rot18 = Rot::new("• Hello Friend 83110 :) •", RotType::Rot18);

        assert_eq!(String::from("• Uryyb Sevraq 38665 :) •"), rot18.encipher());
    }


    #[test]
    fn rot18_decipher() {
        let rot18 = Rot::new("• Uryyb Sevraq 38665 :) •", RotType::Rot18);

        assert_eq!(String::from("• Hello Friend 83110 :) •"), rot18.decipher());
    }


    #[test]
    fn rot13_encipher() {
        let rot13 = Rot::new("• Hello Friend 83110 :) •", RotType::Rot13);

        assert_eq!(String::from("• Uryyb Sevraq 83110 :) •"), rot13.encipher());
    }


    #[test]
    fn rot13_decipher() {
        let rot13 = Rot::new("• Uryyb Sevraq 83110 :) •", RotType::Rot13);

        assert_eq!(String::from("• Hello Friend 83110 :) •"), rot13.decipher());
    }


    #[test]
    fn rot5_encipher() {
        let rot5 = Rot::new("• Hello Friend 83110 :) •", RotType::Rot5);

        assert_eq!(String::from("• Hello Friend 38665 :) •"), rot5.encipher());
    }


    #[test]
    fn rot5_decipher() {
        let rot5 = Rot::new("• Hello Friend 38665 :) •", RotType::Rot5);

        assert_eq!(String::from("• Hello Friend 83110 :) •"), rot5.decipher());
    }
}
