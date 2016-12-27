use tag::*;
use std::*;

pub struct TagValue {
    pub tag: Tag,
    pub value: Vec<u8>,
    bytes: Vec<u8>
}

impl TagValue {

    pub fn new(tag_val: Tag, value: &[u8]) -> TagValue {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend(tag_val.to_num().to_string().as_bytes().iter().cloned());
        bytes.extend("=".as_bytes().iter().cloned());
        bytes.extend(value.iter().cloned());
        bytes.extend("".as_bytes().iter().cloned());

        let mut  value_vec: Vec<u8> = vec![];
        value_vec.extend(value);
        TagValue{tag: tag_val, value: value_vec, bytes: bytes}
    }

    pub fn init(&mut self, tag_val: Tag, value: &[u8]) {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend(tag_val.to_num().to_string().as_bytes().iter().cloned());
        bytes.extend("=".as_bytes().iter().cloned());
        bytes.extend(value.iter().cloned());
        bytes.extend("".as_bytes().iter().cloned());

        let mut  value_vec: Vec<u8> = vec![];
        value_vec.extend(value);

        self.value = value_vec;
        self.bytes = bytes;
        self.tag = tag_val;
    }

    pub fn parse(raw_bytes: &[u8]) -> Result<TagValue, String> {
        let sep_index = match raw_bytes.iter().position(|&c| c == b'=') {
            Some(index) => index,
            None => usize::max_value()
        };

        if sep_index == usize::max_value() {
          return Err("TagValue.parse: '=' not found".to_string())
        }

        let tag_string = str::from_utf8(raw_bytes).unwrap();
        let tag_numstring = &tag_string[0..sep_index];
        let tag_num = tag_numstring.parse::<i32>().unwrap();
        let value_bytes = (&tag_string[(sep_index + 1)..]).as_bytes();
        let tag_val = match Tag::from_number(tag_num) {

            Some(tag) => tag,
            None => return Err(format!("Unknown tag number {}", tag_num))
        };

        let mut bytes: Vec<u8> = vec![];
        bytes.extend(raw_bytes);

        let mut value_vec: Vec<u8> = vec![];
        value_vec.extend(value_bytes);

        Ok(TagValue{tag: tag_val, value: value_vec, bytes: bytes})
    }

    pub fn len(self:TagValue) -> usize {
        self.bytes.len()
    }
}

#[cfg(test)]
mod test {

  use super::*; 
  use tag::*;

    #[test]
    fn new_test() {
        let expected_value= "blahblah".as_bytes();
        let expected_data= "8=blahblah".as_bytes();
        let tag_value = TagValue::new(Tag::BeginString, expected_value);
        assert_eq!(expected_value, tag_value.value.as_slice());
        assert_eq!(expected_data, tag_value.bytes.as_slice());
    }

    #[test]
    fn init_test() {
        let mut tag_value = TagValue{tag:Tag::BeginString, value:vec![], bytes:vec![]};
        let expected_value= "blahblah".as_bytes();
        let expected_data= "8=blahblah".as_bytes();
        tag_value.init(Tag::BeginString, expected_value);
        assert_eq!(expected_value, tag_value.value.as_slice());
        assert_eq!(expected_data, tag_value.bytes.as_slice());
    }

    #[test]
    fn parse_test() {
        let value = "35=A".as_bytes();
        let result = TagValue::parse(value);
        let res = match result {
           Err(ref er) => false,
          _ =>  true
        };
        assert!(res == true);
        assert!("A" == String::from_utf8(result.unwrap().value).unwrap());
    }

}