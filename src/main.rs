//mod generated;
mod page;

use seed::{prelude::*, *};

const POINT: &str = "point";
const QUIZ: &str = "quiz";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        page: Page::init(url),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    page: Page,
}

// ------ Context ------

pub struct Context {
    pub logged_user: &'static str,
}

// ------ Page ------

enum Page {
    Home,
    Quiz(page::quiz::Model),
    Point(page::point::Model),
    NotFound,
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some(QUIZ) => page::quiz::init(url).map_or(Self::NotFound, Self::Quiz),
            Some(POINT) => page::point::init(url).map_or(Self::NotFound, Self::Point),
            Some(_) => Self::NotFound,
        }
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![match &model.page {
        Page::Home => div![
            div![
                C!["text-green-500"],
                p!["Azra Quiz"],
                p!["Test your general knowledge with 10 questions"]
            ],
            button![
                "Start Quiz 🎲",
                ev(Ev::Click, |_| Url::new().set_path(&["quiz"]).go_and_load())
            ],
            button![
                "See points",
                ev(Ev::Click, |_| Url::new().set_path(&["point"]).go_and_load())
            ]
        ],
        Page::Quiz(quiz_model) => page::quiz::view(quiz_model),
        Page::Point(point_model) => page::point::view(point_model),
        Page::NotFound => div!["404"],
    }]
}

// ------ ------
//     Start
// ------ ------

pub fn main() {
    App::start("app", init, update, view);
}
