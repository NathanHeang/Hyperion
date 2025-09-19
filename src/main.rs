#![allow(non_snake_case)]
//#![windows_subsystem = "windows"]

use xilem::view::{button, flex, label, textbox};
use xilem::{EventLoop, WidgetView, Xilem};
use winit::error::EventLoopError;
use scraper::{Html, Selector};

#[derive(Default)]
struct ScraperState {
    url: String,
    selector: String
}

fn scrape(url: String, selector: String){
    let response = reqwest::blocking::get(
        url).unwrap().text().unwrap();

    let doc = Html::parse_document(&response);

    let title = Selector::parse(&*selector).unwrap();

    for title in doc.select(&title) {
        let titles = title.text().collect::<Vec<_>>();
        println!("{}", titles[0])
    }
}

fn appLogic(data:&mut ScraperState) -> impl WidgetView<ScraperState> + use<>{
    flex((
            label(format!("Pasting scraped text into console from {} elements in {}", data.selector, data.url)),
            textbox(data.url.clone(), |data: &mut ScraperState, url|{
                data.url = url
            }),
            textbox(data.selector.clone(), |data: &mut ScraperState, selector|{
                data.selector = selector
            }),
            button("Scrape", |data: &mut ScraperState| scrape(data.url.clone(), data.selector.clone())),
        ))
}

fn main() -> Result<(), EventLoopError> {
    let window = Xilem::new(ScraperState::default(), appLogic);
    window.run_windowed(EventLoop::with_user_event(), "Hyperion".into())?;
    Ok(())
}