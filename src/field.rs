use tag::*;
use std::*;
use tag_value::*;
use std::fmt::Error;

//FieldValueWriter is an interface for writing field values
pub trait FieldValueWriter {
    fn write() -> [u8];
}

//FieldValueReader is an interface for reading field values
pub trait FieldValueReader {
    fn read(&[u8]) -> Result<(), Error>;
}

//The FieldValue interface is used to write/extract typed field values to/from raw bytes
pub trait FieldValue : FieldValueWriter + FieldValueReader {

}

//FieldWriter is an interface for a writing a field
pub trait FieldWriter : FieldValueWriter {
    fn tag() -> Tag;
}

//Field is the interface implemented by all typed Fields in a Message
pub trait FieldInterface : FieldWriter + FieldValueReader {
}

//FieldGroupWriter is an interface for writing a FieldGroup
pub trait FieldGroupWriter {
    fn tag() -> Tag;
    fn write() -> [TagValue];
}

//FieldGroupReader is an interface for reading a FieldGroup
pub trait FieldGroupReader {
    fn tag() -> Tag;
    fn read(tag_value:&[TagValue]) -> Result<&[TagValue], Error>;
}

//FieldGroup is the interface implemented by all typed Groups in a Message
pub trait FieldGroup {
    fn tag() -> Tag;
    fn write() -> [TagValue];
    fn read(tag_value:&[TagValue]) -> Result<&[TagValue], Error>;
}