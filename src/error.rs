use tag::*;
use std::error::Error;

pub enum Reject {
    RejectReasonInvalidTagNumber = 0,
    RejectReasonRequiredTagMissing = 1,
    RejectReasonTagNotDefinedForThisMessageType = 2,
    RejectReasonUnsupportedMessageType = 3,
    RejectReasonTagSpecifiedWithoutAValue = 4,
    RejectReasonValueIsIncorrect = 5,
    RejectReasonConditionallyRequiredFieldMissing = 5,
    RejectReasonIncorrectDataFormatForValue = 6,
    RejectReasonCompIDProblem = 9,
    RejectReasonSendingTimeAccuracyProblem = 10,
    RejectReasonInvalidMsgType = 11,
    RejectReasonTagAppearsMoreThanOnce = 13,
    RejectReasonTagSpecifiedOutOfRequiredOrder = 14,
    RejectReasonRepeatingGroupFieldsOutOfOrder = 15,
    RejectReasonIncorrectNumInGroupCountForRepeatingGroup = 16
}

trait MessageRejectError : Error {
    fn reject_reason(&self) -> i8;
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
        reject_reason: i8,
        description: String,
        ref_tag_id: Tag,
        id_business_reject: bool
    }

    impl MessageRejectError {

        pub fn new_message_reject_error(err: String, reject_reason: Reject, ref_tag_id: &Tag) -> MessageRejectError {
            MessageRejectError{description: err, reject_reason: reject_reason, ref_tag_id: ref_tag_id, id_business_reject: false}
        }

        pub fn new_business_message_reject_error(err: String, reject_reason: Reject, ref_tag_id: &Tag) -> MessageRejectError {
            MessageRejectError{description: err, reject_reason: reject_reason, ref_tag_id: ref_tag_id, id_business_reject: true}
        }

        pub fn conditionally_required_field_missing(tag:Tag) -> MessageRejectError {
            Self::new_message_reject_error(format!("Conditionally Required Field Missing {}", tag), Reject::RejectReasonConditionallyRequiredFieldMissing, &tag)
        }

        pub fn incorrect_data_format_for_value(tag:Tag) -> MessageRejectError {
            Self::new_message_reject_error("Incorrect data format for value", Reject::RejectReasonIncorrectDataFormatForValue, &tag)
        }
    }

    impl super::MessageRejectError for MessageRejectError {
        fn reject_reason(&self) -> i8 { self.reject_reason }
        fn ref_tag_id(&self) -> &Tag { &self.ref_tag_id }
        fn is_business_reject(&self) -> bool { self.id_business_reject }
    }

    impl Error for MessageRejectError {
        fn description(&self) -> &str { self.description.as_str() }
    }

    impl Display  for MessageRejectError {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "({})", self.description)
        }
    }
}