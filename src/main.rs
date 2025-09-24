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
    SelectSourceDir,
    SelectTargetDir,
    Album(String),
}

#[derive(Clone, Debug)]
struct Album {
    name: String,
    path: PathBuf,
    selected: bool,
}

#[derive(Default)]
struct MyApp {
    counter: usize,
    error_message: String,
    source_folder: Option<PathBuf>,
    target_folder: Option<PathBuf>,
    source_albums: Vec<Album>,
    target_albums: Vec<Album>,
}

fn load_albums(folder: Option<PathBuf>) -> Result<Vec<Album>, String> {
    match &folder {
        None => return Err("Could not open directory".to_string()),
        Some(f) => match fs::read_dir(f.display().to_string()) {
            Err(_) => return Err("Could not load directory".to_string()),
            Ok(r) => {
                return Ok(r
                    .filter_map(|entry| entry.ok())
                    .filter_map(|entry| {
                        Some(Album {
                            name: entry.file_name().into_string().unwrap(),
                            path: entry.path(),
                            selected: false,
                        })
                    })
                    .collect())
            }
        },
    };
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::SelectSourceDir => {
                self.source_folder = FileDialog::new().set_directory("~").pick_folder();
                match load_albums(self.source_folder.clone()) {
                    Err(e) => self.error_message = e,
                    Ok(albums) => self.source_albums = albums,
                }
            }
            Message::SelectTargetDir => {
                self.target_folder = FileDialog::new().set_directory("~").pick_folder();
                match load_albums(self.target_folder.clone()) {
                    Err(e) => self.error_message = e,
                    Ok(albums) => self.target_albums = albums,
                }
            }
            Message::Album(album_name) => self.select_album(album_name),
        }
    }

    fn select_album(&mut self, selected_album: String) {
        self.source_albums = self
            .source_albums
            .iter()
            .map(|entry| {
                if entry.name == selected_album {
                    Album {
                        name: entry.name.clone(),
                        path: entry.path.clone(),
                        selected: !entry.selected,
                    }
                } else {
                    entry.clone()
                }
            })
            .collect()
    }

    fn view(&self) -> iced::Element<Message> {
        let test = self.source_albums.iter().map(|e| {
            let style = if e.selected {
                button::danger
            } else {
                button::primary
            };

            iced::widget::Button::new(iced::widget::Text::new(e.name.clone()))
                .on_press(Message::Album(e.name.clone()))
                .style(style)
                .into()
        });

        let mut window = iced::widget::Column::with_children(test);

        window = window.push(button("Select Source Directory").on_press(Message::SelectSourceDir));
        window = window.push(button("Select Target Directory").on_press(Message::SelectTargetDir));
        return window.into();
    }
}
