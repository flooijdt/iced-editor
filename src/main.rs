use iced::{widget::text_editor, Element, Sandbox, Settings};

fn main() -> Result<(), iced::Error> {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

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
        // calling 'into' here  will turn any widget into a generic widget.
        text_editor(&self.content).into()
    }
}
