use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::stringify;
use casey::*;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, pick_list, row, text, checkbox};
use iced::{Element, Length, Renderer, Task, Theme};
use iced_widget::Container;

#[derive(Debug, Clone, PartialEq, Default)]
enum Tab {
    #[default]
    Intro,
    Search,
    ConfigFile,
    Builder,
    Theme,
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
    ThemeChanged(Theme),
}

#[derive(Default)]
struct Tabs {
    active_tab: Tab,
    theme: Theme,
}

impl Tabs {
    fn new() -> (Self, Task<Message>) {
        // Load the theme from a file
        let theme_path = Path::new("theme.txt");

        let theme = match std::fs::read_to_string(theme_path) {
            Ok(contents) => match contents.to_lowercase().as_str() {
                lower!(stringify!(Theme::Light)) => Theme::Light,
                lower!(stringify!(Theme::Dark)) => Theme::Dark,
                lower!(stringify!(Theme::Dracula)) => Theme::Dracula,
                lower!(stringify!(Theme::Nord)) => Theme::Nord,
                lower!(stringify!(Theme::SolarizedLight)) => Theme::SolarizedLight,
                lower!(stringify!(Theme::SolarizedDark)) => Theme::SolarizedDark,
                lower!(stringify!(Theme::GruvboxLight)) => Theme::GruvboxLight,
                lower!(stringify!(Theme::GruvboxDark)) => Theme::GruvboxDark,
                lower!(stringify!(Theme::CatppuccinLatte)) => Theme::CatppuccinLatte,
                lower!(stringify!(Theme::CatppuccinFrappe)) => Theme::CatppuccinFrappe,
                lower!(stringify!(Theme::CatppuccinMacchiato)) => Theme::CatppuccinMacchiato,
                lower!(stringify!(Theme::CatppuccinMocha)) => Theme::CatppuccinMocha,
                lower!(stringify!(Theme::TokyoNight)) => Theme::TokyoNight,
                lower!(stringify!(Theme::TokyoNightStorm)) => Theme::TokyoNightStorm,
                lower!(stringify!(Theme::TokyoNightLight)) => Theme::TokyoNightLight,
                lower!(stringify!(Theme::KanagawaWave)) => Theme::KanagawaWave,
                lower!(stringify!(Theme::KanagawaDragon)) => Theme::KanagawaDragon,
                lower!(stringify!(Theme::KanagawaLotus)) => Theme::KanagawaLotus,
                lower!(stringify!(Theme::Moonfly)) => Theme::Moonfly,
                lower!(stringify!(Theme::Nightfly)) => Theme::Nightfly,
                lower!(stringify!(Theme::Oxocarbon)) => Theme::Oxocarbon,
                lower!(stringify!(Theme::Ferra)) => Theme::Ferra,
                _ => Theme::Dark,
            },
            Err(_) => Theme::Dark,
        };

        (
            Self {
                active_tab: Tab::Intro,
                theme,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => {
                self.active_tab = tab;
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme.clone();

                // Save the theme to a file
                let theme_path = Path::new("theme.txt");
                let mut theme_file = File::create(theme_path).unwrap();
                theme_file.write_all(format!("Theme :: {:?}", theme).as_bytes()).unwrap();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // Create tab buttons
        let tab_buttons = row![
            button("Intro").on_press(Message::TabSelected(Tab::Intro)),
            button("Search").on_press(Message::TabSelected(Tab::Search)),
            button("Config File").on_press(Message::TabSelected(Tab::ConfigFile)),
            button("Builder").on_press(Message::TabSelected(Tab::Builder)),
            button("Theme").on_press(Message::TabSelected(Tab::Theme)),
        ]
        .spacing(4);

        // Create content based on active tab
        let content: Container<'_, Message, Theme, Renderer> = match self.active_tab {
            Tab::Intro => {
                let column = column![
                    text("Welcome to Wynnbuilder Tools"),
                    text("This is a utility application for Wynncraft players."),
                    text("This utility UI is designed to be used in conjunction with the Wynnbuilder tool binaries, builder and search_item respectively."),
                    text("Above you will find tabs for each of the tools, and most importantly, a theme selector."),
                    // Add status checkboxes
                    row![
                        checkbox(
                            "Builder binary found",
                            is_builder_binary_found(),  // TODO: Replace with actual check
                        ).spacing(10),
                    ],
                    row![
                        checkbox(
                            "Search binary found", 
                            is_search_binary_found(),  // TODO: Replace with actual check
                        ).spacing(10),
                    ],
                    row![
                        checkbox(
                            "Config file found",
                            is_config_file_found(),  // TODO: Replace with actual check
                        ).spacing(10),
                    ],
                ]
                .spacing(20)
                .align_x(Horizontal::Center);

                container(column)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            Tab::Search => container(column![text("Content for Search Tab")]).into(),
            Tab::Theme => {
                let selector = column![
                    text("Select Theme:"),
                    pick_list(
                        vec![
                            Theme::Light,
                            Theme::Dark,
                            Theme::Dracula,
                            Theme::Nord,
                            Theme::SolarizedLight,
                            Theme::SolarizedDark,
                            Theme::GruvboxLight,
                            Theme::GruvboxDark,
                            Theme::CatppuccinLatte,
                            Theme::CatppuccinFrappe,
                            Theme::CatppuccinMacchiato,
                            Theme::CatppuccinMocha,
                            Theme::TokyoNight,
                            Theme::TokyoNightStorm,
                            Theme::TokyoNightLight,
                            Theme::KanagawaWave,
                            Theme::KanagawaDragon,
                            Theme::KanagawaLotus,
                            Theme::Moonfly,
                            Theme::Nightfly,
                            Theme::Oxocarbon,
                            Theme::Ferra,
                        ],
                        Some(self.theme.clone()),
                        Message::ThemeChanged
                    )
                ];

                container(selector)
                    .align_x(Horizontal::Left)
                    .align_y(Vertical::Top)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            Tab::ConfigFile => container(column![text("Content for Config File Tab")]).into(),
            Tab::Builder => container(column![text("Content for Builder Tab")]).into(),
        };

        // Main layout
        column![
            tab_buttons,
            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20),
        ]
        .spacing(20)
        .into()
    }
}

fn is_builder_binary_found() -> bool {
    use std::path::Path;
    let builder_names = ["builder", "builder.exe"];
    builder_names.iter().any(|name| Path::new(name).exists())
}

fn is_search_binary_found() -> bool {
    let search_names = ["search_item", "search_item.exe"];
    search_names.iter().any(|name| Path::new(name).exists())
}

fn is_config_file_found() -> bool {
    let config_names = ["config.toml"];
    config_names.iter().any(|name| Path::new("config").join(name).exists())
}

fn main() -> iced::Result {
    iced::application("Wynnbuilder Tools UI", Tabs::update, Tabs::view)
        .theme(|app: &Tabs| app.theme.clone())
        .run_with(Tabs::new)
}
