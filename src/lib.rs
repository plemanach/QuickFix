#[macro_use] extern crate enum_primitive;
extern crate num;
mod tag;
mod tag_value;
mod field_map;
mod field;
mod error;
mod fix_string;
mod fix_boolean;
mod fix_int;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
