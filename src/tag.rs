use num::FromPrimitive;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum  Tag {
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

impl Tag {

  pub fn from_number(num:i32) -> Option<Tag> {
    Tag::from_i32(num)
  }

  pub fn to_num(&self) -> u16 {
    *self as u16
  }
  
  fn is_trailer(&self) -> bool {
    match *self {
      Tag::SignatureLength => true,
      Tag::Signature => true,
      Tag::CheckSum => true,
      _ => false
    }
  }

  fn is_header(&self) -> bool {
      match *self {
        Tag::BeginString  => true,
        Tag::BodyLength  => true,
        Tag::MsgType => true,
        Tag::SenderCompID => true,
        Tag::TargetCompID => true,
        Tag::OnBehalfOfCompID => true,
        Tag::DeliverToCompID => true,
        Tag::SecureDataLen => true,
        Tag::MsgSeqNum => true,
        Tag::SenderSubID => true,
        Tag::SenderLocationID => true,
        Tag::TargetSubID => true,
        Tag::TargetLocationID => true,
        Tag::OnBehalfOfSubID => true,
        Tag::OnBehalfOfLocationID => true,
        Tag::DeliverToSubID => true,
        Tag::DeliverToLocationID => true,
        Tag::PossDupFlag => true,
        Tag::PossResend => true,
        Tag::SendingTime => true,
        Tag::OrigSendingTime => true,
        Tag::XMLDataLen => true,
        Tag::XMLData => true,
        Tag::MessageEncoding => true,
        Tag::LastMsgSeqNumProcessed => true,
        Tag::OnBehalfOfSendingTime => true,
        Tag::ApplVerID => true,
        Tag::CstmApplVerID => true,
        Tag::NoHops => true,
        Tag::ApplExtID => true,
        Tag::SecureData => true,
        Tag::HopCompID => true,
        Tag::HopSendingTime => true,
        Tag::HopRefID => true,
        _ => false
      }
  }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({})", self.to_num())
    }
}

#[cfg(test)]
mod test {

  use super::*; 

  #[test]
  fn is_trailer_test() {
      assert!(Tag::CheckSum.is_trailer());
  }

  #[test]
  fn is_header_test() {
      assert!(Tag::HopCompID.is_header());
  }

  #[test]
  fn to_num_test() {
      assert!(Tag::CheckSum.to_num() == 10);
  }
}
