use std::{
    fs::{self, ReadDir},
    path::PathBuf,
};

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
    error_message: String,
    source_folder: Option<PathBuf>,
    source_albums: Vec<String>,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => self.load_albums(),
        }
    }

    fn load_albums(&mut self) {
        self.source_folder = FileDialog::new().set_directory("~").pick_folder();

        match &self.source_folder {
            None => {}
            Some(f) => match fs::read_dir(f.display().to_string()) {
                Err(_) => self.error_message = "Could not load directory".to_string(),
                Ok(r) => {
                    self.source_albums = r
                        .filter_map(|entry| entry.ok())
                        .filter_map(|entry| {
                            entry.file_name().into_string().ok() // Skip if the filename is not valid UTF-8
                        })
                        .collect()
                }
            },
        };
    }

    fn view(&self) -> iced::Element<Message> {
        let test = self
            .source_albums
            .iter()
            .map(|e| iced::widget::Text::new(e).into());

        let mut window = iced::widget::Column::with_children(test);
        window = window.push(iced::widget::Text::new("!"));
        window = window.push(button("Increase").on_press(Message::ButtonPressed));
        return window.into();
    }
}
