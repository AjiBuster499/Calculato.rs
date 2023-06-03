use iced::{
    executor,
    keyboard::{self, KeyCode},
    subscription,
    widget::{button, column, container, row, text, Text},
    window, Application, Command, Element, Event, Theme,
};

use crate::calculator::Calculator;

pub(crate) struct App {
    calculator: Calculator,
    scientific: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    None,
    Calculate,
    SendToEquation(String),
    Event(Event),
    Scientific,
    Clear,
    Exit,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                calculator: Calculator::new(),
                scientific: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::None => Command::none(),
            Message::Calculate => {
                self.calculator.push_to_equation(" )");
                self.calculator.calculate();
                Command::none()
            }
            Message::SendToEquation(value) => {
                self.calculator.push_to_equation(value.as_str());
                Command::none()
            }
            Message::Exit => window::close(),
            Message::Scientific => {
                self.scientific = !self.scientific;
                Command::none()
            }
            Message::Clear => {
                self.calculator.clear();
                Command::none()
            }
            Message::Event(e) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = e {
                    match key_code {
                        KeyCode::Key1 | KeyCode::Numpad1 => {
                            self.calculator.push_to_equation("1");
                        }
                        KeyCode::Key2 | KeyCode::Numpad2 => {
                            self.calculator.push_to_equation("2");
                        }
                        KeyCode::Key3 | KeyCode::Numpad3 => {
                            self.calculator.push_to_equation("3");
                        }
                        KeyCode::Key4 | KeyCode::Numpad4 => {
                            self.calculator.push_to_equation("4");
                        }
                        KeyCode::Key5 | KeyCode::Numpad5 => {
                            self.calculator.push_to_equation("5");
                        }
                        KeyCode::Key6 | KeyCode::Numpad6 => {
                            self.calculator.push_to_equation("6");
                        }
                        KeyCode::Key7 | KeyCode::Numpad7 => {
                            self.calculator.push_to_equation("7");
                        }
                        KeyCode::Key8 | KeyCode::Numpad8 => {
                            self.calculator.push_to_equation("8");
                        }
                        KeyCode::Key9 | KeyCode::Numpad9 => {
                            self.calculator.push_to_equation("9");
                        }
                        KeyCode::Key0 | KeyCode::Numpad0 => {
                            self.calculator.push_to_equation("0");
                        }
                        KeyCode::Escape => {
                            self.calculator.clear();
                        }
                        KeyCode::Backspace => {
                            self.calculator.backspace();
                        }
                        KeyCode::Enter
                        | KeyCode::NumpadEnter
                        | KeyCode::Equals
                        | KeyCode::NumpadEquals => {
                            self.calculator.push_to_equation(" )");
                            self.calculator.calculate();
                        }
                        KeyCode::Asterisk | KeyCode::NumpadMultiply => {
                            self.calculator.push_to_equation(" * ");
                        }
                        KeyCode::Minus | KeyCode::NumpadSubtract => {
                            self.calculator.push_to_equation(" - ");
                        }
                        KeyCode::Period | KeyCode::NumpadDecimal => {
                            self.calculator.push_to_equation(".");
                        }
                        KeyCode::Plus | KeyCode::NumpadAdd => {
                            self.calculator.push_to_equation(" + ");
                        }
                        KeyCode::Slash | KeyCode::NumpadDivide => {
                            self.calculator.push_to_equation(" / ");
                        }
                        _ => (),
                    };
                }
                Command::none()
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::events().map(Message::Event)
    }

    fn view(&self) -> Element<Message> {
        let equation: Text = text(self.calculator.display_equation.clone());
        let content = column![
            equation,
            row![
                button(text("Calculate")).on_press(Message::Calculate),
                button(text("Clear")).on_press(Message::Clear),
                button(text("Scientific")).on_press(Message::Scientific),
                button(text("Quit")).on_press(Message::Exit),
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
