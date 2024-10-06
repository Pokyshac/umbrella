#![allow(
    dead_code,
    unused_imports,
    unused_variables
)]

use iced::Result;

use umbrella::Umbrella;

mod umbrella;
mod ui;

fn main() -> Result {
    iced::application(Umbrella::title, 
        Umbrella::update, 
        Umbrella::view
    ).run()
}
