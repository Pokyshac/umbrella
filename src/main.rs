#![allow(
    unused_imports,
    unused_braces,
    unused_variables,
    unused_must_use,
    unused_mut,
    dead_code
)]

use fltk::{
    self, app, button::Button, enums::{Color, FrameType}, group::Column, prelude::{GroupExt, WidgetBase, WidgetExt}, text::TextDisplay, window::Window
};

mod umbrella;

fn main() {
    let mut app = umbrella::Umbrella::new(app::Scheme::Base, (200, 200));
    app.get_window()
        .resizable(app.get_window());
    let button = Button::new(10, 10, 90, 140, "flugenheimer");
    let label = TextDisplay::new(120, 50, 60, 60, "asd");
    app.add_widget(button);
    app.add_widget(label);
    app.run().unwrap();
}
