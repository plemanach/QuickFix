use num::FromPrimitive;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Copy, Clone)]
pub struct Tag {
  value:u32
}

impl Display for Tag {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "({})", self.val())
  }
}

impl Tag {

  pub fn new(val:u32) -> Tag {
    Tag{value:val}
  }

  pub fn new_with_define_tag(val:Tags) -> Tag {
    Tag{value:val.to_num()}
  }

  pub fn val(&self) -> u32 {
    self.value
  }

  pub fn val_str(&self) -> String {
    self.value.to_string()
  }

  fn is_trailer(&self) -> bool {
      if let Some(tag) = Tags::from_number(self.value) {
          let result = match tag {
            Tags::SignatureLength => true,
            Tags::Signature => true,
            Tags::CheckSum => true,
            _ => false
          };
          return result;
      }
      false
  }

  fn is_header(&self) -> bool {
      if let Some(tag) = Tags::from_number(self.value) {
          let result = match tag {
            Tags::BeginString => true,
            Tags::BodyLength => true,
            Tags::MsgType => true,
            Tags::SenderCompID => true,
            Tags::TargetCompID => true,
            Tags::OnBehalfOfCompID => true,
            Tags::DeliverToCompID => true,
            Tags::SecureDataLen => true,
            Tags::MsgSeqNum => true,
            Tags::SenderSubID => true,
            Tags::SenderLocationID => true,
            Tags::TargetSubID => true,
            Tags::TargetLocationID => true,
            Tags::OnBehalfOfSubID => true,
            Tags::OnBehalfOfLocationID => true,
            Tags::DeliverToSubID => true,
            Tags::DeliverToLocationID => true,
            Tags::PossDupFlag => true,
            Tags::PossResend => true,
            Tags::SendingTime => true,
            Tags::OrigSendingTime => true,
            Tags::XMLDataLen => true,
            Tags::XMLData => true,
            Tags::MessageEncoding => true,
            Tags::LastMsgSeqNumProcessed => true,
            Tags::OnBehalfOfSendingTime => true,
            Tags::ApplVerID => true,
            Tags::CstmApplVerID => true,
            Tags::NoHops => true,
            Tags::ApplExtID => true,
            Tags::SecureData => true,
            Tags::HopCompID => true,
            Tags::HopSendingTime => true,
            Tags::HopRefID => true,
            _ => false
          };
          return result;
    }
    false
  }
}

enum_from_primitive! {

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]

pub enum  Tags {
  BeginString             = 8,
  BodyLength              = 9,
  MsgType                 = 35,
  SenderCompID            = 49,
  TargetCompID            = 56,
  OnBehalfOfCompID        = 115,
  DeliverToCompID         = 128,
  SecureDataLen           = 90,
  MsgSeqNum               = 34,
  SenderSubID             = 50,
  SenderLocationID        = 142,
  TargetSubID             = 57,
  TargetLocationID        = 143,
  OnBehalfOfSubID         = 116,
  OnBehalfOfLocationID    = 144,
  DeliverToSubID          = 129,
  DeliverToLocationID     = 145,
  PossDupFlag             = 43,
  PossResend              = 97,
  SendingTime             = 52,
  OrigSendingTime         = 122,
  XMLDataLen              = 212,
  XMLData                 = 213,
  MessageEncoding         = 347,
  LastMsgSeqNumProcessed  = 369,
  OnBehalfOfSendingTime   = 370,
  ApplVerID               = 1128,
  CstmApplVerID           = 1129,
  NoHops                  = 627,
  ApplExtID               = 1156,
  SecureData              = 91,
  HopCompID               = 628,
  HopSendingTime          = 629,
  HopRefID                = 630,

  HeartBtInt            = 108,
  BusinessRejectReason  = 380,
  SessionRejectReason   = 373,
  RefMsgType            = 372,
  RefID              = 371,
  RefSeqNum             = 45,
  EncryptMethod         = 98,
  ResetSeqNumFlag       = 141,
  DefaultApplVerID      = 1137,
  Text                  = 58,
  TestReqID             = 112,
  GapFillFlag           = 123,
  NewSeqNo              = 36,
  BeginSeqNo            = 7,
  EndSeqNo              = 16,

  SignatureLength  = 93,
  Signature        = 89,
  CheckSum         = 10
}
}


impl Tags {

  pub fn from_number(num:u32) -> Option<Tags> {
    Tags::from_u32(num)
  }

  pub fn to_num(&self) -> u32 {
    *self as u32
  }
}


impl Display for Tags {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({})", *self)
    }
}

#[cfg(test)]
mod test {

  use super::*; 

  #[test]
  fn is_trailer_test() {

      let tag = Tag::new_with_define_tag(Tags::CheckSum);
      assert!(tag.is_trailer());
  }

  #[test]
  fn is_header_test() {
      let tag = Tag::new_with_define_tag(Tags::HopCompID);
      assert!(tag.is_header());
  }

  #[test]
  fn to_num_test() {
      assert!(Tags::CheckSum.to_num() == 10);
  }
}
