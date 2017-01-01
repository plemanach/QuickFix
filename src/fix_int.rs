use std::*;
use field::*;
use error::{FixError};

impl FieldValueReader for i32 {
    fn read(&mut self, value: &[u8]) -> Result<(), FixError> {
        let mut values: Vec<u8> = vec![];
        values.extend(value.iter().cloned());

        let number_string = match String::from_utf8(values) {
            Ok(str) => str,
            Err(err) => return Err(FixError::FromUtf8Error(err))
        };

        match number_string.parse::<i32>()  {
            Ok(number) => *self = number,
            Err(err) => return Err(FixError::IntError(err))
        };

        Ok(())
    }
}

impl FieldValueWriter for i32 {
    fn write(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}