#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parser);

mod app;
mod calculator;

// fn main() -> iced::Result {
//     <app::App as iced::Application>::run(iced::Settings::default())
// }

fn main() {
    match parser::ExprParser::new().parse("2+2") {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Error: {}", e),
    }
}
