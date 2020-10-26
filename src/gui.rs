use crate::{Priklad, Result};
use iced::{text_input, Row, Sandbox, Text, TextInput};

#[derive(Default)]
pub struct RScalc {
    input_state: text_input::State,
    input_field: String,
    vysledok: String,
}

impl Sandbox for RScalc {
    type Message = Message;

    fn new() -> Self {
        RScalc {
            input_state: text_input::State::focused(),
            ..RScalc::default()
        }
    }

    fn title(&self) -> String {
        "RScalc".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(mut priklad) => {
                self.input_field = priklad.clone();
                priklad.push('\n'); // TODO: fix so this is not needed
                let priklad: Result<Priklad> = priklad.parse();
                if let Ok(priklad) = priklad {
                    if let Ok(vysledok) = priklad.solve() {
                        self.vysledok = format!("= {}", vysledok);
                    } else {
                        self.vysledok = "Cannot calculate".into()
                    }
                } else {
                    self.vysledok = "Bad equation".into()
                };
            }
        };
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let input = TextInput::new(
            &mut self.input_state,
            "Enter the problem",
            &self.input_field,
            Message::InputChanged,
        )
        .padding(4);
        let vysledok = Text::new(self.vysledok.clone())
            .height(iced::Length::Fill)
            .vertical_alignment(iced::VerticalAlignment::Center)
            .size(24);
        Row::new()
            .padding(10)
            .spacing(8)
            .push(input)
            .push(vysledok)
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
}
