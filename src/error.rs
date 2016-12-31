use tag::*;
use std::{string, fmt};
use std;
use std::error::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Reject {
    InvalidTagNumber = 0,
    RequiredTagMissing = 1,
    TagNotDefinedForThisMessageType = 2,
    UnsupportedMessageType = 3,
    TagSpecifiedWithoutAValue = 4,
    ValueIsIncorrect = 5,
    ConditionallyRequiredFieldMissing = 6,
    IncorrectDataFormatForValue = 7,
    CompIDProblem = 8,
    SendingTimeAccuracyProblem = 9,
    InvalidMsgType = 11,
    TagAppearsMoreThanOnce = 13,
    SpecifiedOutOfRequiredOrder = 14,
    RepeatingGroupFieldsOutOfOrder = 15,
    IncorrectNumInGroupCountForRepeatingGroup = 16
}

#[derive(Debug)]
pub struct FixBooleanParseError {
    description : String,
    value_parsed: String
}

impl FixBooleanParseError {
    pub fn new(value:String) -> FixBooleanParseError {
        FixBooleanParseError{description: format!("Boolean could not be parsed:{}",value), value_parsed:value}
    }
}

impl Error for FixBooleanParseError {
    fn description(&self) -> &str { self.description.as_str() }
}

impl std::fmt::Display for FixBooleanParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.description())
    }
}

#[derive(Debug)]
pub enum FixError {
    Error(fmt::Error),
    FromUtf8Error(string::FromUtf8Error),
    BooleanParseError(FixBooleanParseError)
}

impl std::error::Error for FixError {
    fn description(&self) -> &str {
        match *self {
            FixError::Error(ref err) => err.description(),
            FixError::FromUtf8Error(ref err) => err.description(),
            FixError::BooleanParseError(ref err) => err.description(),
        }
    }
}

impl std::fmt::Display for FixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FixError::Error(ref err) => write!(f, "({})", self.description()),
            FixError::FromUtf8Error(ref err) => write!(f, "({})", self.description()),
            FixError::BooleanParseError(ref err) => write!(f, "({})", self.description())
        }
    }
}

trait MessageRejectError : std::error::Error {
    fn reject_reason(&self) -> Reject;
    fn ref_tag_id(&self) -> &Tag;
    fn is_business_reject(&self) -> bool;
}

pub mod error {

    use tag::*;
    use super::*;
    use std::error::Error;
    use std::fmt::Display;
    use std::fmt::Formatter;
    use std::fmt::Result;

    #[derive(Debug)]
    pub struct MessageRejectError {
        reject_reason: Reject,
        description: String,
        ref_tag_id: Tag,
        id_business_reject: bool
    }

    impl MessageRejectError {

        pub fn new_message_reject_error(err: String, reject_reason: Reject, ref_tag_id: Tag) -> MessageRejectError {
            MessageRejectError{description: err, reject_reason: reject_reason, ref_tag_id: ref_tag_id, id_business_reject: false}
        }

        pub fn new_business_message_reject_error(err: String, reject_reason: Reject, ref_tag_id: Tag) -> MessageRejectError {
            MessageRejectError{description: err, reject_reason: reject_reason, ref_tag_id: ref_tag_id, id_business_reject: true}
        }

        pub fn conditionally_required_field_missing(tag:Tag) -> MessageRejectError {
            Self::new_message_reject_error(format!("Conditionally Required Field Missing {}", tag), Reject::ConditionallyRequiredFieldMissing, tag)
        }

        pub fn incorrect_data_format_for_value(tag:Tag) -> MessageRejectError {
            Self::new_message_reject_error("Incorrect data format for value".to_string(), Reject::IncorrectDataFormatForValue, tag)
        }
    }

    impl super::MessageRejectError for MessageRejectError {
        fn reject_reason(&self) -> Reject { self.reject_reason }
        fn ref_tag_id(&self) -> &Tag { &self.ref_tag_id }
        fn is_business_reject(&self) -> bool { self.id_business_reject }
    }

    impl Error for MessageRejectError {
        fn description(&self) -> &str { self.description.as_str() }
    }

    impl Display for MessageRejectError {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "({})", self.description)
        }
    }
}