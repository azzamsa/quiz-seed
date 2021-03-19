use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

type Model = i32;

// ------ ------
//    Update
// ------ ------

enum Msg {}

fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}

// ------ ------
//     View
// ------ ------

fn view(_: &Model) -> Node<Msg> {
    div![p!["Hello world!"]]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
