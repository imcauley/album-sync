use std::{
    collections::HashSet,
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
    error_message: String,
    source_folder: Option<PathBuf>,
    target_folder: Option<PathBuf>,
    source_albums: Vec<Album>,
    target_albums: Vec<Album>,
}

fn load_albums(folder: Option<PathBuf>) -> Result<Vec<Album>, String> {
    let folder = folder.ok_or_else(|| "Could not open directory".to_string())?;

    let entries = fs::read_dir(&folder).map_err(|_| "Could not load directory".to_string())?;

    let albums = entries
        .filter_map(Result::ok)
        .map(|entry| Album {
            name: entry.file_name().into_string().unwrap_or_default(),
            path: entry.path(),
            selected: false,
        })
        .collect();

    Ok(albums)
}

fn diff_on_albums_list(source_albums: Vec<Album>, target_albums: Vec<Album>) -> Vec<Album> {
    if source_albums.is_empty() {
        return source_albums;
    }

    if target_albums.is_empty() {
        return source_albums;
    }

    let target_hash: HashSet<String> = target_albums
        .iter()
        .map(|album| album.name.clone())
        .collect();

    source_albums
        .iter()
        .map(|entry| {
            if target_hash.contains(&entry.name) {
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

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::SelectSourceDir => {
                self.source_folder = FileDialog::new().set_directory("~").pick_folder();
                match load_albums(self.source_folder.clone()) {
                    Err(e) => self.error_message = e,
                    Ok(albums) => self.source_albums = albums,
                }
                self.source_albums =
                    diff_on_albums_list(self.source_albums.clone(), self.target_albums.clone());
            }
            Message::SelectTargetDir => {
                self.target_folder = FileDialog::new().set_directory("~").pick_folder();
                match load_albums(self.target_folder.clone()) {
                    Err(e) => self.error_message = e,
                    Ok(albums) => self.target_albums = albums,
                }
                self.source_albums =
                    diff_on_albums_list(self.source_albums.clone(), self.target_albums.clone());
            }
            Message::Album(album_name) => self.select_album(album_name),
        }
    }

    fn select_album(&mut self, selected_album: String) {
        self.source_albums = self
            .source_albums
            .iter()
            .map(|entry| {
                let mut album = entry.clone();
                if album.name == selected_album {
                    album.selected = !album.selected;
                }
                album
            })
            .collect();
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
        return iced::widget::scrollable(window).into();
    }
}
