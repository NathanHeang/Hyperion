#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use druid::widget::{Align, Button, Flex, Label, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Env, Lens, Widget, WindowDesc};
use scraper::{Html, Selector};
#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

fn scrape(url: String){
    let response = reqwest::blocking::get(
        url).unwrap().text().unwrap();

    let doc_body = Html::parse_document(&response);

    let title = Selector::parse("td").unwrap();

    for title in doc_body.select(&title) {
        let titles = title.text().collect::<Vec<_>>();
        println!("{}", titles[0])
    }
}

fn main() {
    let window = WindowDesc::new(buildRootWidget())
        .title("Hyperion")
        .window_size((400.0, 400.0));

    let stateInit = HelloState{
        name: " ".into(),
    };

    AppLauncher::with_window(window)
        .launch(stateInit)
        .expect("Failed to initialize Hyperion");
}

fn  buildRootWidget() -> impl Widget<HelloState> {
    let label = Label::new(|data : &HelloState, _env: &Env| format!("Pasting scraped text into console from {}", data.name));

    let textBox = TextBox::new()
        .with_placeholder("Test")
        .fix_width(200.0)
        .lens(HelloState::name);

    let button = Button::new("Scrape")
        .on_click(|_, data: &mut HelloState, _env| {
            scrape(data.name.clone())
        });

    let layout = Flex::column()
        .with_child(label)
        .with_spacer(10.0)
        .with_child(textBox)
        .with_child(button);
    Align::centered(layout)
}