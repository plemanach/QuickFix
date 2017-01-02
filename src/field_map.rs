use tag::*;
use std::*;
use tag_value::*;
use std::collections::*;
use field::*;
use error::error::MessageRejectError;
use fix_string::*;
use fix_boolean::*;
use fix_int::*;

pub struct Field {
     field: Vec<TagValue>
}

impl Field {

    fn new() -> Field {
        Field{field:vec![TagValue::empty()]}
    }

    fn field_tag(&self) -> Tag{
        self.field[0].tag()
    }

    fn init_field(&mut self, tag:Tag, value:&[u8])
    {
        self.field[0].init(tag, value);
    }
}

type TagOrder = fn(i:Tag, j:Tag) -> bool;

pub struct TagSort {
    tags: Vec<Tag>,
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
    tag_lookup: HashMap<Tag, Field>,
    tag_sort: TagSort
}

impl FieldMap {

    // ascending tags
    fn normal_field_order(i:Tag, j:Tag) -> bool { i < j }

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

    fn lookup_field(&self, tag:Tag) -> Option<&Field> {
        self.tag_lookup.get(&tag)
    }

    fn get_field<T>(&self, tag:Tag,  parser: &mut T)-> Result<(),MessageRejectError> where T: FieldValueReader {
        let mut field = match self.tag_lookup.get(&tag) {
            Some(f) => f,
            None => return Err(MessageRejectError::conditionally_required_field_missing(tag))
        };

        match parser.read(field.field[0].value()) {
            Err(_)  => return Err(MessageRejectError::incorrect_data_format_for_value(tag)),
            _ => Ok(())
        }
    }

    fn get_bytes(&self, tag:Tag) -> Result<&[u8] ,MessageRejectError> {
        if !self.tag_lookup.contains_key(&tag) {
            return Err(MessageRejectError::conditionally_required_field_missing(tag));
        }
        Ok(self.tag_lookup.get(&tag).unwrap().field[0].value())
    }

    fn get_or_create(&mut self, tag:Tag) -> &mut Field
    {
        if !self.tag_lookup.contains_key(&tag) {
            self.tag_lookup.insert(tag, Field::new());
            self.tag_sort.tags.push(tag);
        }
        self.tag_lookup.get_mut(&tag).unwrap()
    }

    fn set_field<T:FieldValueWriter>(&mut self, tag:Tag, field: T) {
        self.set_bytes(tag, field.write().as_ref())
    }

    fn set_bytes(&mut self, tag:Tag, value: &[u8]) {
        let mut f = self.get_or_create(tag);
        f.init_field(tag, value);
    }

    fn set_int(&mut self, tag:Tag, value:i32) {
        self.set_bytes(tag, value.write().as_ref());
    }

    fn set_string(&mut self, tag:Tag, value:&str) {
        self.set_bytes(tag, value.as_ref());
    }

    //Get parses out a field in this FieldMap. Returned reject may indicate the field is not present, or the field value is invalid.
    fn get<T>(&self, parser: &mut T) -> Result<(),MessageRejectError> where T: FieldInterface {
        return self.get_field(parser.tag(), parser)
    }

    fn get_string(&self, tag:Tag) -> Result<String, MessageRejectError> {
        let mut value = FIXString::new();
        {
            let value_mutable = &mut value;
            match self.get_field(tag, value_mutable) {
                Err(e) => return Err(e),
                _ => true
            };
        }
        Ok(value.into())
    }

    fn get_boolean(&self, tag:Tag) -> Result<bool, MessageRejectError> {
        let mut value = FIXBoolean::new();
        {
            let value_mutable = &mut value;
            match self.get_field(tag, value_mutable) {
                Err(e) => return Err(e),
                _ => true
            };
        }
        Ok(value.into())
    }

    fn get_int(&self, tag:Tag) -> Result<i32, MessageRejectError> {
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
}

#[cfg(test)]
mod test {
    use std::collections::*;
    use super::*;
    use tag_value::*;
    use tag::*;
    #[test]
    fn add_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "blahblah".as_bytes();
        let tag_value = TagValue::new(Tag::BeginString, expected_value);
        field_map.add(Field{field: vec![tag_value]});
        let field_count = field_map.tags().len();
        assert_eq!(1, field_count);
    }
    #[test]
    fn get_string_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "blahblah".as_bytes();
        let tag_value = TagValue::new(Tag::BeginString, expected_value);
        field_map.add(Field{field: vec![tag_value]});
        assert_eq!("blahblah", field_map.get_string(Tag::BeginString).unwrap());
    }
    #[test]
    fn get_string_true_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "Y".as_bytes();
        let tag_value = TagValue::new(Tag::PossDupFlag, expected_value);
        field_map.add(Field{field: vec![tag_value]});
        assert_eq!(true, field_map.get_boolean(Tag::PossDupFlag).unwrap());
    }
    #[test]
    fn get_string_false_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "N".as_bytes();
        let tag_value = TagValue::new(Tag::PossDupFlag, expected_value);
        field_map.add(Field{field: vec![tag_value]});
        assert_eq!(false, field_map.get_boolean(Tag::PossDupFlag).unwrap());
    }
    #[test]
    fn get_int_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "11".as_bytes();
        let tag_value = TagValue::new(Tag::MsgSeqNum, expected_value);
        field_map.add(Field{field: vec![tag_value]});
        assert_eq!(11, field_map.get_int(Tag::MsgSeqNum).unwrap());
    }
    #[test]
    fn get_negative_int_test() {
        let mut field_map = FieldMap::new();
        let expected_value= "-2".as_bytes();
        let tag_value = TagValue::new(Tag::MsgSeqNum, expected_value);
        field_map.add(Field{field: vec![tag_value]});
        assert_eq!(-2, field_map.get_int(Tag::MsgSeqNum).unwrap());
    }
    #[test]
    fn typed_set_and_get() {
        let mut field_map = FieldMap::new();

        let expected_string_1 =  "Sender1";
        let expected_string_2 =  "Receiver1";
        field_map.set_string(Tag::SenderCompID, expected_string_1);
        field_map.set_string(Tag::TargetCompID, expected_string_2);

        assert_eq!(expected_string_1,  field_map.get_string(Tag::SenderCompID).unwrap());
        assert_eq!(expected_string_2,  field_map.get_string(Tag::TargetCompID).unwrap());
        //assert!(field_map.get_string(Tag::BodyLength).is_err() == true);
        let result = field_map.get_string(Tag::BodyLength);
    }
}