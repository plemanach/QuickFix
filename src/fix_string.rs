use std::*;
use field::*;
use error::FixError;

impl FieldValueReader for String {
    fn read(&mut self, value: &[u8]) -> Result<(), FixError> {
        let mut values: Vec<u8> = vec![];
        values.extend(value.iter().cloned());

        match String::from_utf8(values) {
            Ok(str) => *self = str,
            Err(err) => return Err(FixError::FromUtf8Error(err))
        }
        Ok(())
    }
}

impl FieldValueWriter for String {
    fn write(&self) -> Vec<u8> {
        self.clone().into_bytes()
    }
}