use tag::*;
use std::*;
use tag_value::*;

struct Field {
     field: Vec<TagValue>
 }

impl Field {

  fn fieldTag(&self) -> Tag{

      self.field[0].tag
  }

}
