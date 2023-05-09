use iced::{executor, Application, Command, Element, Theme};

use crate::calculator::Calculator;

#[derive(Debug)]
pub(crate) struct App {
    _calculator: Calculator,
}

#[derive(Debug)]
pub(crate) enum Messages {}

impl Application for App {
    type Executor = executor::Default;

    type Message = Messages;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                _calculator: Calculator::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Messages> {
        todo!();
    }
}
