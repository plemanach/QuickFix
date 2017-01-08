use std::*;
use field::*;
use error::{FixBooleanParseError, FixError};


impl FieldValueReader for bool {
    fn read(&mut self, value: &[u8]) -> Result<(), FixError> {
        let mut values: Vec<u8> = vec![];
        values.extend(value.iter().cloned());

        let bool_string = match String::from_utf8(values) {
            Ok(str) => str,
            Err(err) => return Err(FixError::FromUtf8Error(err))
        };

        match bool_string.as_str() {
            "Y" => *self = true,
            "N" => *self = false,
            _ => return Err(FixError::BooleanParseError(FixBooleanParseError::new(bool_string)))
        }
        Ok(())
    }
}

impl FieldValueWriter for bool {
    fn write(&self) -> Vec<u8> {
        if *self {
            "Y".to_string().into_bytes()
        }else
        {
            "N".to_string().into_bytes()
        }
    }
}