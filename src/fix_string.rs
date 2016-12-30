use std::*;
use field::*;
use error::FixError;

pub struct FixString {
    value: String
}

impl FixString {
    pub fn new() -> FixString
    {
        FixString{value: String::new()}
    }

    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }

    pub fn into(self) -> String {
        self.value
    }
}

impl FieldValueReader for FixString {
    fn read(&mut self, value: &[u8]) -> Result<(), FixError> {
        let mut values: Vec<u8> = vec![];
        values.extend(value.iter().cloned());

        match String::from_utf8(values) {
            Ok(str) => self.value = str,
            Err(err) => return Err(FixError::FromUtf8Error(err))
        }
        Ok(())
    }
}

impl FieldValueWriter for FixString {
    fn write(&self) -> Vec<u8> {
        self.value.clone().into_bytes()
    }
}