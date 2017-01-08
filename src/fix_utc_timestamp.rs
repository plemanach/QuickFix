use std::*;
use field::*;
use error::FixError;
use time::*;
use std::iter::FromIterator;


static UTCTIMESTAMPFORMAT: &'static str = "%Y-%m-%d %H:%M:%S.%f";
static UTCTIMESTAMPNOMILLISFORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub struct FIXUTCTimestamp {
    time: Tm,
    no_millisecond: bool
}

impl FIXUTCTimestamp {

    pub fn new(time:Tm) -> FIXUTCTimestamp {
        FIXUTCTimestamp{time:time, no_millisecond:false}
    }

    pub fn new_no_millis(time:Tm, no_millis:bool) -> FIXUTCTimestamp {
        FIXUTCTimestamp{time:time, no_millisecond:no_millis}
    }

    pub fn empty() -> FIXUTCTimestamp {
        FIXUTCTimestamp{time:empty_tm(), no_millisecond:false}
    }

    pub fn into(self) -> Tm {
        self.time
    }
}

impl FieldValueReader for FIXUTCTimestamp {
    fn read(&mut self, value: &[u8]) -> Result<(), FixError> {

        let mut values: Vec<u8> = vec![];
        values.extend(value.iter().cloned());

        let str_time = match String::from_utf8(values) {
            Ok(str) => str,
            Err(err) => return Err(FixError::FromUtf8Error(err))
        };

        if let Ok(time) = strptime(str_time.as_str(), UTCTIMESTAMPFORMAT)
        {
            self.time = time;
            return Ok(());
        }

        match strptime(str_time.as_str(), UTCTIMESTAMPNOMILLISFORMAT)
        {
            Ok(time) => self.time = time,
            Err(err) => return Err(FixError::TimeParseError(err))
        }
        self.no_millisecond = true;

        Ok(())
    }
}

impl FieldValueWriter for FIXUTCTimestamp {
    fn write(&self) -> Vec<u8> {
        if self.no_millisecond {
            strftime(UTCTIMESTAMPNOMILLISFORMAT, &self.time).unwrap().into_bytes()
        } else {
            strftime(UTCTIMESTAMPFORMAT, &self.time).unwrap().into_bytes()
        }
    }
}