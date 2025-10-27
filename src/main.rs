#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use xilem::view::{button, flex, label, portal, prose, sized_box, textbox, Axis};
use xilem::{Color, EventLoop, WidgetView, Xilem};
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
    sized_box(
        portal(
            flex((
                    label(format!("Pasting scraped text into console from {} elements in {}", data.selector, data.url)),
                    textbox(data.url.clone(), |data: &mut AppData, url|{
                        data.url = url
                    }),
                    textbox(data.selector.clone(), |data: &mut AppData, selector|{
                        data.selector = selector
                    }),
                    button("Scrape", |data: &mut AppData| data.output =  scrape(data.url.clone(), data.selector.clone())),
                    prose(format!("{}", data.output))
            ))
                .gap(6.25)
                .direction(Axis::Vertical)
        )
    )
        .border(Color::from_rgb8(255, 255, 225), 1.25)
}

fn main() -> Result<(), EventLoopError> {
    Xilem::new(AppData::default(), appLogic)
        .background_color(Color::from_rgb8(40, 5, 20))
        .run_windowed(EventLoop::with_user_event(), "Hyperion".into())?;
    Ok(())
}