#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parser);

mod app;
mod calculator;

use app::App;
use iced::{window, Application, Settings};

fn main() -> iced::Result {
    App::run(Settings {
        id: Some(String::from("CalculatoRS")),
        window: window::Settings {
            decorations: true,
            size: (300, 300),
            ..Default::default()
        },
        ..Settings::default()
    })
}
