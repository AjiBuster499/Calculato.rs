#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parser);

mod app;
mod calculator;

fn main() -> iced::Result {
    <app::App as iced::Application>::run(iced::Settings::default())
}
