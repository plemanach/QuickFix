use tag::*;
use std::*;
use tag_value::*;
use std::collections::*;
use field::*;
use error::error::MessageRejectError;

pub struct Field {
     field: Vec<TagValue>
}

impl Field {

    fn new() -> Field {
        Field{field:vec![]}
    }

    fn field_tag(&self) -> Tag{
        self.field[0].tag
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

    fn get_field(&self, tag:Tag, parser:FieldValueReader) -> Result<(),MessageRejectError> {
        let field = match self.tag_lookup.get(tag) {
            Some(f) => f,
            None => return MessageRejectError::conditionally_required_field_missing(tag)
        };

        match parser.Read(field[0].value) {
            Err(_)  => return  MessageRejectError::incorrect_data_format_for_value(tag),
            _ => Ok()
        }
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
}