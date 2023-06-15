use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    executor,
    keyboard::{self, KeyCode},
    subscription,
    widget::{button, column, container, row, text, Text},
    window, Application, Command, Element, Event, Length, Theme,
};

use crate::calculator::Calculator;

pub(crate) struct App {
    calculator: Calculator,
    display_equation: String,
    scientific: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    None,
    Calculate,
    SendToEquation(char),
    Event(Event),
    Scientific,
    Clear,
    Backspace,
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
                calculator: Calculator,
                display_equation: String::from(" "),
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
                let answer = self.calculator.calculate(&self.display_equation);
                self.display_equation = answer.to_string();
                Command::none()
            }
            Message::SendToEquation(c) => {
                self.display_equation.push(c);
                Command::none()
            }
            Message::Exit => window::close(),
            Message::Scientific => {
                self.scientific = !self.scientific;
                Command::none()
            }
            Message::Clear => {
                self.display_equation.clear();
                Command::none()
            }
            Message::Backspace => {
                self.display_equation.pop();
                Command::none()
            }
            // TODO: Find a better way to do this.
            Message::Event(e) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = e {
                    match key_code {
                        KeyCode::Key1 | KeyCode::Numpad1 => {
                            self.display_equation.push('1');
                        }
                        KeyCode::Key2 | KeyCode::Numpad2 => {
                            self.display_equation.push('2');
                        }
                        KeyCode::Key3 | KeyCode::Numpad3 => {
                            self.display_equation.push('3');
                        }
                        KeyCode::Key4 | KeyCode::Numpad4 => {
                            self.display_equation.push('4');
                        }
                        KeyCode::Key5 | KeyCode::Numpad5 => {
                            self.display_equation.push('5');
                        }
                        KeyCode::Key6 | KeyCode::Numpad6 => {
                            self.display_equation.push('6');
                        }
                        KeyCode::Key7 | KeyCode::Numpad7 => {
                            self.display_equation.push('7');
                        }
                        KeyCode::Key8 | KeyCode::Numpad8 => {
                            self.display_equation.push('8');
                        }
                        KeyCode::Key9 | KeyCode::Numpad9 => {
                            self.display_equation.push('9');
                        }
                        KeyCode::Key0 | KeyCode::Numpad0 => {
                            self.display_equation.push('0');
                        }
                        KeyCode::Escape => {
                            self.display_equation.clear();
                        }
                        KeyCode::Backspace => {
                            self.display_equation.pop();
                        }
                        KeyCode::Enter
                        | KeyCode::NumpadEnter
                        | KeyCode::Equals
                        | KeyCode::NumpadEquals => {
                            let answer = self.calculator.calculate(&self.display_equation);
                            self.display_equation = answer.to_string();
                        }
                        KeyCode::Plus | KeyCode::NumpadAdd => {
                            self.display_equation.push('+');
                        }
                        KeyCode::Minus | KeyCode::NumpadSubtract => {
                            self.display_equation.push('-');
                        }
                        KeyCode::Asterisk | KeyCode::NumpadMultiply => {
                            self.display_equation.push('*');
                        }
                        KeyCode::Slash | KeyCode::NumpadDivide => {
                            self.display_equation.push('/');
                        }
                        KeyCode::Period | KeyCode::NumpadDecimal => {
                            self.display_equation.push('.');
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
        let equation: Text = text(&self.display_equation)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .width(Length::Fill);
        // Row containing the Equation
        let eq_row = row![
            equation,
            button(text("X").horizontal_alignment(Horizontal::Center)).on_press(Message::Exit)
        ];
        let content = column![
            eq_row,
            row![
                button(text("C").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::Clear)
                    .width(Length::Fill),
                button(text("Sci").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::Scientific)
                    .width(Length::Fill),
                button(text("/").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('/'))
                    .width(Length::Fill),
            ]
            .align_items(Alignment::Center),
            row![
                button(text("1").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('1'))
                    .width(Length::Fill),
                button(text("2").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('2'))
                    .width(Length::Fill),
                button(text("3").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('3'))
                    .width(Length::Fill),
                button(text("+").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('+'))
                    .width(Length::Fill),
            ]
            .align_items(Alignment::Center),
            row![
                button(text("4").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('4'))
                    .width(Length::Fill),
                button(text("5").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('5'))
                    .width(Length::Fill),
                button(text("6").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('6'))
                    .width(Length::Fill),
                button(text("-").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('-'))
                    .width(Length::Fill),
            ]
            .align_items(Alignment::Center),
            row![
                button(text("7").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('7'))
                    .width(Length::Fill),
                button(text("8").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('8'))
                    .width(Length::Fill),
                button(text("9").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('9'))
                    .width(Length::Fill),
                button(text("*").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('*'))
                    .width(Length::Fill),
            ]
            .align_items(Alignment::Center),
            row![
                // // TODO: Implement Negatives?
                // button(text("+/-"))
                //     .on_press(Message::None)
                //     .width(Length::Fill),
                button(text("0").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('0'))
                    .width(Length::Fill),
                button(text(".").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation('.'))
                    .width(Length::Fill),
                button(text("<-").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::Backspace)
                    .width(Length::Fill),
                button(text("=").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::Calculate)
                    .width(Length::Fill),
            ]
            .align_items(Alignment::Center)
        ]
        .align_items(Alignment::Center);
        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
