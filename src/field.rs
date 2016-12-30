use tag::*;
use tag_value::*;
use error::FixError;

//FieldValueWriter is an interface for writing field values
pub trait FieldValueWriter {
    fn write(&self) -> Vec<u8>;
}

//FieldValueReader is an interface for reading field values
pub trait FieldValueReader {
    fn read(&mut self, &[u8]) -> Result<(), FixError>;
}

//The FieldValue interface is used to write/extract typed field values to/from raw bytes
pub trait FieldValue : FieldValueWriter + FieldValueReader {

}

//FieldWriter is an interface for a writing a field
pub trait FieldWriter : FieldValueWriter {
    fn tag(&self) -> Tag;
}

//Field is the interface implemented by all typed Fields in a Message
pub trait FieldInterface : FieldWriter + FieldValueReader {
}

//FieldGroupWriter is an interface for writing a FieldGroup
pub trait FieldGroupWriter {
    fn tag(&self) -> Tag;
    fn write(&self) -> [TagValue];
}

//FieldGroupReader is an interface for reading a FieldGroup
pub trait FieldGroupReader {
    fn tag(&self) -> Tag;
    fn read(&self, tag_value:&[TagValue]) -> Result<&[TagValue], FixError>;
}

//FieldGroup is the interface implemented by all typed Groups in a Message
pub trait FieldGroup {
    fn tag(&self) -> Tag;
    fn write(&self) -> [TagValue];
    fn read(&self, tag_value:&[TagValue]) -> Result<&[TagValue], FixError>;
}