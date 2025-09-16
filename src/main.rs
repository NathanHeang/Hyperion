#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use druid::widget::{Align, Button, Flex, Label, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Env, Lens, Widget, WindowDesc};
use scraper::{Html, Selector};
#[derive(Clone, Data, Lens)]
struct ScraperState {
    url: String,
    selector: String
}

fn scrape(url: String, selector: String){
    let response = reqwest::blocking::get(
        url).unwrap().text().unwrap();

    let doc_body = Html::parse_document(&response);

    let title = Selector::parse(&*selector).unwrap();

    for title in doc_body.select(&title) {
        let titles = title.text().collect::<Vec<_>>();
        println!("{}", titles[0])
    }
}

fn main() {
    let window = WindowDesc::new(buildRootWidget())
        .title("Hyperion")
        .window_size((400.0, 400.0));

    let stateInit = ScraperState{
        url: "".into(),
        selector: "".into()
    };

    AppLauncher::with_window(window)
        .launch(stateInit)
        .expect("Failed to initialize Hyperion");
}

fn  buildRootWidget() -> impl Widget<ScraperState> {
    let label = Label::new(|data : &ScraperState, _env: &Env| format!("Pasting scraped text into console from {} elements in {}", data.selector, data.url));

    let urlBox = TextBox::new()
        .with_placeholder("URL")
        .fix_width(200.0)
        .lens(ScraperState::url);

    let selectorBox = TextBox::new()
        .with_placeholder("HTML Selector")
        .fix_width(200.0)
        .lens(ScraperState::selector);

    let button = Button::new("Scrape")
        .on_click(|_, data: &mut ScraperState, _env| {
            scrape(data.url.clone(), data.selector.clone())
        });

    let layout = Flex::column()
        .with_child(label)
        .with_spacer(10.0)
        .with_child(urlBox)
        .with_child(selectorBox)
        .with_child(button);
    Align::centered(layout)
}