use iced::widget::{button, column, text};
use rfd::FileDialog;

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

#[derive(Default)]
struct MyApp {
    counter: usize,
    source_folder: String,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                let folder_handler = FileDialog::new().set_directory("~").pick_folder();
                match folder_handler {
                    None => self.source_folder = "".to_string(),
                    Some(f) => self.source_folder = f.display().to_string(),
                }
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            text(self.source_folder.clone()),
            button("Increase").on_press(Message::ButtonPressed),
        ]
        .into()
    }
}
