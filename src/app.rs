use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    executor,
    widget::{button, column, container, row, text, text_input},
    window, Application, Command, Element, Length, Theme,
};

use crate::parser::calculate;

pub(crate) struct App {
    display_equation: String,
    input: String,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Calculate,
    SendToEquation(String),
    // InputChanged(String),
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
                display_equation: String::new(),
                input: String::new(),
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
            Message::Calculate => {
                let answer = calculate(&self.display_equation);
                self.display_equation = answer.to_string();
            }
            Message::SendToEquation(s) => {
                // self.display_equation.push_str(&s);
                self.display_equation = s;
            }
            Message::Exit => return window::close(),
            Message::Clear => {
                self.display_equation.clear();
            }
            Message::Backspace => {
                self.display_equation.pop();
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let equation_out = text(&self.display_equation)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .width(Length::Fill);
        let equation_in = text_input("Enter an equation...", &self.display_equation)
            .on_input(Message::SendToEquation)
            .on_submit(Message::Calculate);
        // Row containing the Equation
        let eq_row = column![
            equation_out,
            row![
                equation_in,
                button(text("X").horizontal_alignment(Horizontal::Center)).on_press(Message::Exit)
            ]
        ];
        // Scientific Functions
        let scientific_block = column![
            row![
                button(text("^").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("^".to_string()))
                    .width(Length::Fill),
                button(text("^2").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("^2".to_string()))
                    .width(Length::Fill),
                button(text("log").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("log(".to_string()))
                    .width(Length::Fill),
                button(text("ln").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("ln(".to_string()))
                    .width(Length::Fill),
                button(text("!").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("!".to_string()))
                    .width(Length::Fill),
            ]
            .align_items(Alignment::Center),
            row![
                button(text("(").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("(".to_string()))
                    .width(Length::Fill),
                button(text(")").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation(")".to_string()))
                    .width(Length::Fill),
                button(text("nPr").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("nPr".to_string()))
                    .width(Length::Fill),
                button(text("nCr").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("nCr".to_string()))
                    .width(Length::Fill),
            ],
        ]
        .align_items(Alignment::Center);
        let content = column![
            eq_row,
            scientific_block,
            row![
                button(text("C").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::Clear)
                    .width(Length::Fill),
                button(text("/").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("/".to_string()))
                    .width(Length::Fill),
            ]
            .align_items(Alignment::Center),
            row![
                button(text("1").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("1".to_string()))
                    .width(Length::Fill),
                button(text("2").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("2".to_string()))
                    .width(Length::Fill),
                button(text("3").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("3".to_string()))
                    .width(Length::Fill),
                button(text("+").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("+".to_string()))
                    .width(Length::Fill),
            ]
            .align_items(Alignment::Center),
            row![
                button(text("4").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("4".to_string()))
                    .width(Length::Fill),
                button(text("5").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("5".to_string()))
                    .width(Length::Fill),
                button(text("6").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("6".to_string()))
                    .width(Length::Fill),
                button(text("-").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("-".to_string()))
                    .width(Length::Fill),
            ]
            .align_items(Alignment::Center),
            row![
                button(text("7").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("7".to_string()))
                    .width(Length::Fill),
                button(text("8").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("8".to_string()))
                    .width(Length::Fill),
                button(text("9").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("9".to_string()))
                    .width(Length::Fill),
                button(text("*").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("*".to_string()))
                    .width(Length::Fill),
            ]
            .align_items(Alignment::Center),
            row![
                button(text("0").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation("0".to_string()))
                    .width(Length::Fill),
                button(text(".").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::SendToEquation(".".to_string()))
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
