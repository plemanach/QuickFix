use tag::*;
use std::*;
use tag_value::*;
use std::collections::*;
use field::*;
use error::error::MessageRejectError;
use fix_string::*;
use fix_boolean::*;
use fix_int::*;
use time::*;
use fix_utc_timestamp::*;

pub struct Field {
     field: Vec<TagValue>
}

impl Field {

    fn new() -> Field {
        Field{field:vec![TagValue::empty()]}
    }

    fn field_tag(&self) -> u32 {
        self.field[0].tag()
    }

    fn init_field(&mut self, tag:u32, value:&[u8])
    {
        self.field[0].init(tag, value);
    }
}

type TagOrder = fn(i:u32, j:u32) -> bool;

pub struct TagSort {
    tags: Vec<u32>,
    compare: TagOrder
}

impl TagSort {

    fn len(&self) -> usize { self.tags.len() }

    fn swap(&mut self, i:usize, j:usize) {
        let tmp = self.tags[i];
        self.tags[i] = self.tags[j];
        self.tags[j] = self.tags[i];
    }

    fn less(&self, i:usize, j:usize) -> bool { self.tags[i].eq(&self.tags[j]) }
}

pub struct FieldMap {
    tag_lookup: HashMap<u32, Field>,
    tag_sort: TagSort
}

impl FieldMap {

    // ascending tags
    fn normal_field_order(i:u32, j:u32) -> bool { i < j }

    fn new() -> FieldMap {
        FieldMap{tag_lookup:HashMap::new(),tag_sort: TagSort{tags: vec![], compare: FieldMap::normal_field_order }}
    }

    fn init(&mut self) {
        self.init_with_ordering(FieldMap::normal_field_order);
    }

    fn init_with_ordering(&mut self, ordering:TagOrder) {
      self.tag_lookup = HashMap::new();
      self.tag_sort.compare = ordering;
    }

    fn tags(&self) -> Vec<&Field> {
        let mut fields:Vec<&Field> = vec![];
        for field in self.tag_lookup.values() {
            fields.push(field);
        }
        return fields;
    }

    fn add(&mut self, f:Field) {
        let tag = f.field_tag();
        if !self.tag_lookup.contains_key(&tag) {
            self.tag_lookup.insert(tag, f);
            self.tag_sort.tags.push(tag);
        } else {
            *self.tag_lookup.entry(tag).or_insert(Field::new()) = f;
        }
    }

    fn lookup_field(&self, tag:u32) -> Option<&Field> {
        self.tag_lookup.get(&tag)
    }

    fn get_field<T>(&self, tag:u32,  parser: &mut T)-> Result<(),MessageRejectError> where T: FieldValueReader {
        let mut field = match self.tag_lookup.get(&tag) {
            Some(f) => f,
            None => return Err(MessageRejectError::conditionally_required_field_missing(tag))
        };

        match parser.read(field.field[0].value()) {
            Err(_)  => return Err(MessageRejectError::incorrect_data_format_for_value(tag)),
            _ => Ok(())
        }
    }

    fn get_bytes(&self, tag:u32) -> Result<&[u8] ,MessageRejectError> {
        if !self.tag_lookup.contains_key(&tag) {
            return Err(MessageRejectError::conditionally_required_field_missing(tag));
        }
        Ok(self.tag_lookup.get(&tag).unwrap().field[0].value())
    }

    fn get_or_create(&mut self, tag:u32) -> &mut Field
    {
        if !self.tag_lookup.contains_key(&tag) {
            self.tag_lookup.insert(tag, Field::new());
            self.tag_sort.tags.push(tag);
        }
        self.tag_lookup.get_mut(&tag).unwrap()
    }

    fn set_field<T:FieldValueWriter>(&mut self, tag:u32, field: T) {
        self.set_bytes(tag, field.write().as_ref())
    }

    fn set_bytes(&mut self, tag:u32, value: &[u8]) {
        let mut f = self.get_or_create(tag);
        f.init_field(tag, value);
    }

    fn set_int(&mut self, tag:u32, value:i32) {
        self.set_bytes(tag, value.write().as_ref());
    }

    fn set_bool(&mut self, tag:u32, value:bool) {
        self.set_bytes(tag, value.write().as_ref());
    }

    fn set_time(&mut self, tag:u32, value:Tm) {
        self.set_bytes(tag, FIXUTCTimestamp::new(value).write().as_ref());
    }

    fn set_string(&mut self, tag:u32, value:&str) {
        self.set_bytes(tag, value.as_ref());
    }

    //Get parses out a field in this FieldMap. Returned reject may indicate the field is not present, or the field value is invalid.
    fn get<T>(&self, parser: &mut T) -> Result<(),MessageRejectError> where T: FieldInterface {
        return self.get_field(parser.tag(), parser)
    }

    fn get_string(&self, tag:u32) -> Result<String, MessageRejectError> {
        let mut value = String::new();
        {
            let value_mutable = &mut value;
            match self.get_field(tag, value_mutable) {
                Err(e) => return Err(e),
                _ => true
            };
        }
        Ok(value.into())
    }

    fn get_bool(&self, tag:u32) -> Result<bool, MessageRejectError> {
        let mut value = false;
        {
            let value_mutable = &mut value;
            match self.get_field(tag, value_mutable) {
                Err(e) => return Err(e),
                _ => true
            };
        }
        Ok(value.into())
    }

    fn get_int(&self, tag:u32) -> Result<i32, MessageRejectError> {
        let mut value: i32 = 0;
        {
            let value_mutable = &mut value;
            match self.get_field(tag, value_mutable) {
                Err(e) => return Err(e),
                _ => true
            };
        }
        Ok(value)
    }

    fn get_time(&self, tag:u32) -> Result<Tm, MessageRejectError> {
        let mut value = FIXUTCTimestamp::empty();
        {
            let value_mutable = &mut value;
            match self.get_field(tag, value_mutable) {
                Err(e) => return Err(e),
                _ => true
            };
        }
        Ok(value.into())
    }
}

#[cfg(test)]
mod test {
    use std::collections::*;
    use super::*;
    use tag_value::*;
    use tag::*;
    use time::*;

    #[test]
    fn add_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "blahblah".as_bytes();
        let tag_value = TagValue::new(Tags::BeginString.into(), expected_value);
        field_map.add(Field{field: vec![tag_value]});
        let field_count = field_map.tags().len();
        assert_eq!(1, field_count);
    }

    #[test]
    fn get_string_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "blahblah".as_bytes();
        let tag_value = TagValue::new(Tags::BeginString.into(), expected_value);
        field_map.add(Field{field: vec![tag_value]});
        assert_eq!("blahblah", field_map.get_string(Tags::BeginString.into()).unwrap());
    }

    #[test]
    fn get_string_true_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "Y".as_bytes();
        let tag_value = TagValue::new(Tags::PossDupFlag.into(), expected_value);
        field_map.add(Field{field: vec![tag_value]});
        assert_eq!(true, field_map.get_bool(Tags::PossDupFlag.into()).unwrap());
    }

    #[test]
    fn get_string_false_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "N".as_bytes();
        let tag_value = TagValue::new(Tags::PossDupFlag.to_num(), expected_value);
        field_map.add(Field{field: vec![tag_value]});
        assert_eq!(false, field_map.get_bool(Tags::PossDupFlag.to_num()).unwrap());
    }

    #[test]
    fn get_int_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "11".as_bytes();
        let tag_value = TagValue::new(Tags::MsgSeqNum.to_num(), expected_value);
        field_map.add(Field{field: vec![tag_value]});
        assert_eq!(11, field_map.get_int(Tags::MsgSeqNum.to_num()).unwrap());
    }

    #[test]
    fn set_bool_test() {
        let mut field_map = FieldMap::new();
        let expected_value = true;
        field_map.set_bool(Tags::PossDupFlag.to_num(),expected_value);
        assert_eq!(expected_value, field_map.get_bool(Tags::PossDupFlag.to_num()).unwrap());
    }

    #[test]
    fn set_time_test() {
        let expected_value = now_utc();
        let mut field_map = FieldMap::new();
        field_map.set_time(Tags::SendingTime.to_num(),expected_value);
        assert_eq!(expected_value.to_timespec(), field_map.get_time(Tags::SendingTime.to_num()).unwrap().to_timespec());
    }

    #[test]
    fn get_negative_int_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "-2".as_bytes();
        let tag_value = TagValue::new(Tags::MsgSeqNum.to_num(), expected_value);
        field_map.add(Field{field: vec![tag_value]});
        assert_eq!(-2, field_map.get_int(Tags::MsgSeqNum.to_num()).unwrap());
    }

    #[test]
    fn typed_set_and_get() {
        let mut field_map = FieldMap::new();

        let expected_string_1 =  "Sender1";
        let expected_string_2 =  "Receiver1";
        field_map.set_string(Tags::SenderCompID.to_num(), expected_string_1);
        field_map.set_string(Tags::TargetCompID.to_num(), expected_string_2);

        assert_eq!(expected_string_1,  field_map.get_string(Tags::SenderCompID.to_num()).unwrap());
        assert_eq!(expected_string_2,  field_map.get_string(Tags::TargetCompID.to_num()).unwrap());
        //assert!(field_map.get_string(Tag::BodyLength).is_err() == true);
        let result = field_map.get_string(Tags::BodyLength.to_num());
    }
}