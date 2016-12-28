use tag::*;
use std::*;
use tag_value::*;
use std::collections::*;

struct Field {
     field: Vec<TagValue>
 }

impl Field {

    fn field_tag(&self) -> Tag{
        self.field[0].tag
    }

    fn init_field(&mut self, tag:Tag, value:&[u8])
    {
        self.field[0].init(tag, value);
    }
}

type TagOrder = fn(i:Tag, j:Tag) -> bool;

struct TagSort {
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

struct FieldMap  {
    tag_lookup: HashMap<Tag, Field>,
    tag_sort: TagSort
}

impl FieldMap {

    // ascending tags
    fn normal_field_order(i:Tag, j:Tag) -> bool { i < j }

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
          *self.tag_lookup.entry(tag).or_insert(Field{field:vec![]}) = f;
      }

    }
}

#[cfg(test)]
mod test {

    use super::*; 

    #[test]
    fn add_test() {

      
    }

}