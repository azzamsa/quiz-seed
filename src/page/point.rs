use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub fn init(_: Url) -> Option<Model> {
    Some(Model {})
}

pub struct Model {}

// ------ ------
//     View
// ------ ------

pub fn view<Ms>(_: &Model) -> Node<Ms> {
    div!["This is point page"]
}
