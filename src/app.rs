use iced::{
    executor,
    widget::{button, column, container, row, text, Text},
    window, Application, Command, Element, Theme,
};

use crate::calculator::Calculator;

#[derive(Debug)]
pub(crate) struct App {
    calculator: Calculator,
    show_scientific: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) enum Message {
    None,
    Calculate,
    SendToEquation(String),
    Scientific,
    Clear,
    Quit,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                calculator: Calculator::new(),
                show_scientific: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::None => unimplemented!(),
            Message::Calculate => {
                self.calculator.push_to_equation(" )");
                self.calculator.calculate();
            }
            Message::SendToEquation(value) => self.calculator.push_to_equation(value.as_str()),
            Message::Quit => return window::close(),
            Message::Scientific => self.show_scientific = !self.show_scientific,
            Message::Clear => self.calculator.clear(),
        };

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let equation: Text = text(self.calculator.display_equation.clone());
        let content = column![
            equation,
            row![
                button(text("Calculate")).on_press(Message::Calculate),
                button(text("Clear")).on_press(Message::Clear),
                button(text("Scientific")).on_press(Message::Scientific),
                button(text("Quit")).on_press(Message::Quit),
            ]
            .align_items(iced::Alignment::Center),
            row![
                button(text("1")).on_press(Message::SendToEquation(String::from("1"))),
                button(text("2")).on_press(Message::SendToEquation(String::from("2"))),
                button(text("3")).on_press(Message::SendToEquation(String::from("3"))),
                button(text("+")).on_press(Message::SendToEquation(String::from(" + "))),
            ]
            .align_items(iced::Alignment::Center),
            row![
                button(text("4")).on_press(Message::SendToEquation(String::from("4"))),
                button(text("5")).on_press(Message::SendToEquation(String::from("5"))),
                button(text("6")).on_press(Message::SendToEquation(String::from("6"))),
                button(text("-")).on_press(Message::SendToEquation(String::from(" - "))),
            ]
            .align_items(iced::Alignment::Center),
            row![
                button(text("7")).on_press(Message::SendToEquation(String::from("7"))),
                button(text("8")).on_press(Message::SendToEquation(String::from("8"))),
                button(text("9")).on_press(Message::SendToEquation(String::from("9"))),
                button(text("*")).on_press(Message::SendToEquation(String::from(" * "))),
            ]
            .align_items(iced::Alignment::Center),
            row![
                // TODO: Implement Negatives?
                button(text("+/-")).on_press(Message::None),
                button(text("0")).on_press(Message::SendToEquation(String::from("0"))),
                button(text(".")).on_press(Message::SendToEquation(String::from("."))),
                button(text("/")).on_press(Message::SendToEquation(String::from(" / "))),
            ]
            .align_items(iced::Alignment::Center),
        ]
        .align_items(iced::Alignment::Center);
        container(content).into()
    }
}
