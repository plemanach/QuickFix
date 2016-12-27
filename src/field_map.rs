use tag::*;
use std::*;
use tag_value::*;

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
