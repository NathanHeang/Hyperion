#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use xilem::view::{button, flex, label, portal, textbox};
use xilem::{EventLoop, WidgetView, Xilem};
use winit::error::EventLoopError;
use scraper::{Html, Selector};

#[derive(Default)]
struct AppData {
    url: String,
    selector: String,
    output: String,
}

fn scrape(url: String, selector: String) -> String {
    let response = reqwest::blocking::get(
        url).unwrap().text().unwrap();
    let doc = Html::parse_document(&response);
    let title = Selector::parse(&*selector).unwrap();
    let mut output = String::new();

    for title in doc.select(&title) {
        let titles = title.text().collect::<Vec<_>>();
        println!("{}", titles[0]);
        output.push_str("\r\n");
        output.push_str(titles[0])
    }
    output
}

fn appLogic(data:&mut AppData) -> impl WidgetView<AppData> + use<>{
    flex((
            label(format!("Pasting scraped text into console from {} elements in {}", data.selector, data.url)),
            textbox(data.url.clone(), |data: &mut AppData, url|{
                data.url = url
            }),
            textbox(data.selector.clone(), |data: &mut AppData, selector|{
                data.selector = selector
            }),
            button("Scrape", |data: &mut AppData| data.output =  scrape(data.url.clone(), data.selector.clone())),
            portal(label(format!("{}", data.output)))
        ))
}

fn main() -> Result<(), EventLoopError> {
    let window = Xilem::new(AppData::default(), appLogic);
    window.run_windowed(EventLoop::with_user_event(), "Hyperion".into())?;
    Ok(())
}