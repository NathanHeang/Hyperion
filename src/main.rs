#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};
use druid::widget::{Align, Flex, Label, TextBox};

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

fn main() {
    let window = WindowDesc::new(buildRootWidget())
        .title("Hyperion")
        .window_size((400.0, 400.0));

    let stateInit = HelloState{
        name: "World".into(),
    };

    AppLauncher::with_window(window)
        .launch(stateInit)
        .expect("Failed to initialize Hyperion");
}

fn  buildRootWidget() -> impl Widget<HelloState> {
    let label = Label::new(|data : &HelloState, _env: &Env| format!("Hello {}!", data.name));

    let text_box = TextBox::new()
        .with_placeholder("Welcome to Hyperion!")
        .fix_width(200.0)
        .lens(HelloState::name);

    let layout = Flex::column()
        .with_child(label)
        .with_spacer(10.0)
        .with_child(text_box);

    Align::centered(layout)
}