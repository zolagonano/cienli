use crate::common::key_gen;

pub struct Xor<'a> {
    key: &'a str,
}

impl Xor<'_> {
    pub fn new(key: &str) -> Xor {
        Xor { key: key }
    }

    pub fn encipher(&self, message: &str) -> String {
        let key = key_gen(&self.key, message.len()).unwrap();

        Xor::xor_engine(message, &key)
    }

    pub fn decipher(&self, cipher: &str) -> String {
        let key = key_gen(&self.key, cipher.len()).unwrap();

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

        assert_eq!(String::from("<=|zv"), xor.encipher("jp14N"));
    }

    #[test]
    fn decipher() {
        let xor = Xor::new("jp14N");

        assert_eq!(String::from("VMMN8"), xor.decipher("<=|zv"));
    }
}
