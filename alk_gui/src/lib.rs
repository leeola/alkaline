use iced::{
    widget::{button, column, text, Column},
    Center,
};

pub mod value {

    pub mod from_json {}

    pub enum Value {
        Row,
        Col,
    }
}

// TODO: Make this entry less main-y and more pluggable, for use in `alk gui` and `alk_gui`. Not
// yet sure how to do that with Iced.
pub fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            },
            Message::Decrement => {
                self.value -= 1;
            },
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}
