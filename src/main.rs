use std::{
    fs::{self},
    path::PathBuf,
};

use iced::widget::button;
use rfd::FileDialog;

fn main() -> iced::Result {
    iced::run("My App", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

struct Album {
    name: String,
    selected: bool,
}

#[derive(Default)]
struct MyApp {
    counter: usize,
    error_message: String,
    source_folder: Option<PathBuf>,
    source_albums: Vec<Album>,
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
            None => self.error_message = "Could not open directory".to_string(),
            Some(f) => match fs::read_dir(f.display().to_string()) {
                Err(_) => self.error_message = "Could not load directory".to_string(),
                Ok(r) => {
                    self.source_albums = r
                        .filter_map(|entry| entry.ok())
                        .filter_map(|entry| {
                            Some(Album {
                                name: entry.file_name().into_string().unwrap(),
                                selected: false,
                            })
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
            .map(|e| iced::widget::Text::new(e.name.clone()).into());

        let mut window = iced::widget::Column::with_children(test);
        window = window.push(button("Increase").on_press(Message::ButtonPressed));
        return window.into();
    }
}
