use std::*;
use field::*;
use error::{FixBooleanParseError, FixError};

pub struct FIXBoolean {
    value: bool
}

impl FIXBoolean {
    pub fn new() -> FIXBoolean {
        FIXBoolean{value:true}
    }

    pub fn into(self) -> bool {
        self.value
    }
}

impl FieldValueReader for FIXBoolean {
    fn read(&mut self, value: &[u8]) -> Result<(), FixError> {
        let mut values: Vec<u8> = vec![];
        values.extend(value.iter().cloned());

        let bool_string = match String::from_utf8(values) {
            Ok(str) => str,
            Err(err) => return Err(FixError::FromUtf8Error(err))
        };

        match bool_string.as_str() {
            "Y" => self.value = true,
            "N" => self.value = false,
            _ => return Err(FixError::BooleanParseError(FixBooleanParseError::new(bool_string)))
        }
        Ok(())
    }
}

impl FieldValueWriter for FIXBoolean {
    fn write(&self) -> Vec<u8> {
        if self.value {
            "Y".to_string().into_bytes()
        }else
        {
            "N".to_string().into_bytes()
        }
    }
}