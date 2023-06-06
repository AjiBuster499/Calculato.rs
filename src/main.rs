#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parser);

mod app;
mod calculator;

use app::App;
use iced::{Application, Settings};

fn main() -> iced::Result {
    App::run(Settings {
        id: Some(String::from("CalculatoRS")),
        ..Settings::default()
    })
}
