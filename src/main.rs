use iced::{widget::text, Element, Sandbox};

fn main() {
    println!("Hello, world!");
}

struct Editor;

#[derive(Debug)]
enum Message {}

impl Sandbox for Editor {
    // a message is basically any event that alters the app's state. (eg: clicking something.).
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("A cool editor.")
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello iced!").into()
    }
}
