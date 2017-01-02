use std::*;
use field::*;
use error::FixError;

pub struct FIXString {
    value: String
}

impl FIXString {

    pub fn new_with_value(value:&str) -> FIXString
    {
        FIXString{value: value.to_string()}
    }

    pub fn new() -> FIXString
    {
        FIXString{value: String::new()}
    }

    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }

    pub fn into(self) -> String {
        self.value
    }
}

impl FieldValueReader for FIXString {
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

impl FieldValueWriter for FIXString {
    fn write(&self) -> Vec<u8> {
        self.value.clone().into_bytes()
    }
}