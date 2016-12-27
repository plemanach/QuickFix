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
    //fn less(&self, i:isize, j:isize) -> bool { self.tags[i]. }
}

struct FieldMap  {
    tag_lookup: HashMap<Tag, Field>,
    tag_sort: TagSort
}