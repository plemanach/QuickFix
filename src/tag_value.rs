use tag::*;
use std::*;

pub struct TagValue {
  
  tag: Tag,
  pub value: Vec<u8>,
  bytes: Vec<u8>
}

impl TagValue {

  fn new(tag_val: Tag, value: &[u8]) -> TagValue  {
      

    let mut bytes: Vec<u8> = vec![];
    
    bytes.extend(tag_val.to_num().to_string().as_bytes().iter().cloned());
    bytes.extend("=".as_bytes().iter().cloned());
    bytes.extend(value.iter().cloned());

    let mut  value_vec: Vec<u8> = vec![];
    value_vec.extend(value);



    TagValue{tag: tag_val, value: value_vec, bytes: bytes}

  }


  fn parse(raw_bytes: &[u8]) -> Result<TagValue, String> {

    let sep_index = match raw_bytes.iter().position(|&c| c == b'=') {
        Some(index) => index,
        None => usize::max_value()
    };

    if sep_index == usize::max_value() {

      return Err(String::from("TagValue.parse: '=' not found"))
    }


    let tag_string = str::from_utf8(raw_bytes).unwrap();
    let tag_numstring = &tag_string[0..sep_index];
    let tag_num = tag_numstring.parse::<i32>().unwrap();
    let value_bytes = (&tag_string[(sep_index + 1)..]).as_bytes();

    let error: String = format!("Unknown tag number {}", tag_num);


    let tag_val = match Tag::from_number(tag_num) {

        Some(tag) => tag,
        None => return Err(error)
    };

    let mut bytes: Vec<u8> = vec![];
    bytes.extend(raw_bytes);

    let mut value_vec: Vec<u8> = vec![];
    value_vec.extend(value_bytes);


    Ok(TagValue{tag: tag_val, value: value_vec, bytes: bytes})

  }

  pub fn len(self:TagValue) -> usize  {

      self.bytes.len()
  }
}


#[cfg(test)]
mod test {

  use super::*; 
  use tag::*;


  #[test]
  fn new_test()  {
    
    let value = "A".as_bytes();
    let tag_value = TagValue::new(Tag::HopRefID, value);
    assert!(tag_value.len() == 5)
  }

  #[test]
  fn parse_test()  {

    let value = "35=A".as_bytes();
    let result = TagValue::parse(value);
    let res = match result {
      Err(er) => er,
      _ => String::from("ok")
    };

    assert!(res == "ok");
  }


}