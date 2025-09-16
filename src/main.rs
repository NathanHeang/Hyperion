#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use druid::widget::prelude::*;
use druid::widget::{Align, Flex, Label, TextBox, Painter, WidgetExt};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, Rect, Color};

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

fn main() {
    let window = WindowDesc::new(buildRootWidget())
        .title("Hyperion")
        .window_size((400.0, 400.0));

    let stateInit = HelloState{
        name: "Enter Your Name".into(),
    };

    AppLauncher::with_window(window)
        .launch(stateInit)
        .expect("Failed to initialize Hyperion");
}

fn  buildRootWidget() -> impl Widget<HelloState> {
    let label = Label::new(|data : &HelloState, _env: &Env| format!("Hello, {}!", data.name));

    let textBox = TextBox::new()
        .with_placeholder("Welcome to Hyperion!")
        .fix_width(200.0)
        .lens(HelloState::name);

    let layout = Flex::column()
        .with_child(label)
        .with_spacer(10.0)
        .with_child(textBox);

    Align::centered(layout)
}