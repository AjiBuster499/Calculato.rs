mod app;
mod calculator;

fn main() -> iced::Result {
    <app::App as iced::Application>::run(iced::Settings::default())
}
