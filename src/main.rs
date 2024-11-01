use std::path::Path;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_input};
use iced::{Element, Length, Renderer, Task, Theme};
use iced_widget::text_editor::Action;
use iced_widget::{text_editor, Container};
use serde::{Deserialize, Serialize};

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
    SearchInputChanged(String),
    SearchInputSubmitted,
    SearchItemEditorAction(Action),
    SearchItemStdout(String),
    Ignore, // For readonly text input
    ConfigFileEditorAction(Action),
}

#[derive(Default)]
struct Tabs {
    active_tab: Tab,
    theme: Theme,
    search_input: String,
    search_results: text_editor::Content,
    config_file_content: text_editor::Content,
}
mod theme_serde;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThemeConfig {
    #[serde(with = "theme_serde")]
    theme: Theme,
}

impl Tabs {
    fn new() -> (Self, Task<Message>) {
        // Create settings directory if it doesn't exist
        let settings_dir = Path::new("settings");
        let _ = std::fs::create_dir_all(settings_dir);
        let theme_path = settings_dir.join("theme.toml");

        let theme = match std::fs::read_to_string(&theme_path) {
            Ok(contents) => match toml::from_str::<ThemeConfig>(&contents) {
                Ok(config) => config.theme,
                Err(_) => Theme::Dark,
            },
            Err(_) => Theme::Dark,
        };

        let config_file_content = match std::fs::read_to_string("config/config.toml") {
            Ok(content) => content,
            Err(_) => String::from("# Config file not found.\n# A new one will be created when you save."),
        };

        (
            Self {
                active_tab: Tab::Intro,
                theme,
                search_input: String::new(),
                search_results: text_editor::Content::new(),
                config_file_content: text_editor::Content::with_text(&config_file_content),
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

                // Save the theme to settings folder
                let theme_config = ThemeConfig {
                    theme: theme.clone(),
                };
                let settings_dir = Path::new("settings");
                let theme_path = settings_dir.join("theme.toml");
                let theme_toml = toml::to_string(&theme_config).unwrap();
                let _ = std::fs::write(theme_path, theme_toml);
            }
            Message::SearchInputChanged(input_text) => {
                self.search_input = input_text;
            }
            Message::SearchInputSubmitted => {
                let binary_name = if cfg!(windows) {
                    "search_item.exe"
                } else {
                    "search_item"
                };

                // Split input into arguments, preserving quoted strings if present
                let args: Vec<&str> = self.search_input.split_whitespace().collect();

                let output = match std::process::Command::new(binary_name).args(args).output() {
                    Ok(output) => {
                        match output.status.success() {
                            true => String::from_utf8_lossy(&output.stdout).to_string(),
                            false => String::from_utf8_lossy(&output.stderr).to_string(),
                        }
                    }
                    Err(e) => format!("Error: Could not execute search_item binary: {}", e),
                };

                self.search_results = text_editor::Content::with_text(&output);
            }
            Message::Ignore => (),
            Message::SearchItemStdout(output) => {
                self.search_results = text_editor::Content::with_text(&output);
            }
            Message::SearchItemEditorAction(action) => {
                match action {
                    Action::Edit(_) => (), // Do nothing for edits
                    Action::Move(_) => (), // Do nothing for moves
                    Action::Select(motion) => {
                        self.search_results
                            .perform(Action::Select(motion));
                    }
                    Action::SelectWord => {
                        self.search_results.perform(Action::SelectWord);
                    }
                    Action::SelectLine => {
                        self.search_results.perform(Action::SelectLine);
                    }
                    Action::SelectAll => {
                        self.search_results.perform(Action::SelectAll);
                    }
                    Action::Click(point) => {
                        self.search_results
                            .perform(Action::Click(point));
                    }
                    Action::Drag(point) => {
                        self.search_results
                            .perform(Action::Drag(point));
                    }
                    Action::Scroll { lines } => {
                        self.search_results
                            .perform(Action::Scroll { lines });
                    }
                }
            }
            Message::ConfigFileEditorAction(action) => {
                match action {
                    Action::Edit(edit) => {
                        self.config_file_content.perform(Action::Edit(edit));
                        // Write the updated content to file
                        if let Ok(_) = std::fs::write("config/config.toml", self.config_file_content.text()) {
                            // Successfully saved
                        }
                    },
                    Action::Move(movement) => {
                        // Enable cursor movement
                        self.config_file_content.perform(Action::Move(movement));
                    },
                    Action::Select(motion) => {
                        self.search_results
                            .perform(Action::Select(motion));
                    }
                    Action::SelectWord => {
                        self.search_results.perform(Action::SelectWord);
                    }
                    Action::SelectLine => {
                        self.search_results.perform(Action::SelectLine);
                    }
                    Action::SelectAll => {
                        self.search_results.perform(Action::SelectAll);
                    }
                    Action::Click(point) => {
                        self.search_results
                            .perform(Action::Click(point));
                    }
                    Action::Drag(point) => {
                        self.search_results
                            .perform(Action::Drag(point));
                    }
                    Action::Scroll { lines } => {
                        self.search_results
                            .perform(Action::Scroll { lines });
                    }
                }
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
                            is_builder_binary_found(),
                        ).spacing(10),
                    ],
                    row![
                        checkbox(
                            "Search binary found", 
                            is_search_binary_found(),
                        ).spacing(10),
                    ],
                    row![
                        checkbox(
                            "Config file found",
                            is_config_file_found(),
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
            Tab::Search => {
                let column = column![
                    text("Search for Wynncraft Items")
                        .size(30),
                    text("This tool allows you to search through all Wynncraft items using various parameters.")
                        .size(20),
                    text("By pressing enter, the output will be displayed in the text editor below.")
                        .size(20),
                    text("For a list of available parameters, type '--help' or '-h'.")
                        .size(20),
                    text("For example: '--type boots -s hp -l 10' will show you the 10 highest hp boots.")
                        .size(20),
                    // Search input
                    text_input(
                        "Enter search parameters...",
                        &self.search_input
                    )
                    .on_input(|input| Message::SearchInputChanged(input))
                    .on_submit(Message::SearchInputSubmitted)
                    .padding(10)
                    .width(Length::Fill),
                    text_editor(
                        &self.search_results
                    )
                    .placeholder("Output will appear here...")
                    .on_action(Message::SearchItemEditorAction)/* 
                    .line_height(30.0)
                    .size(16) */
                ]
                .spacing(20)
                .align_x(Horizontal::Center);

                container(column)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Top)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
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
            Tab::ConfigFile => {
                let column = column![
                    text("Edit Configuration File")
                        .size(30),
                    text("The configuration will be saved automatically when you edit.")
                        .size(20),
                    text_editor(
                        &self.config_file_content
                    )
                    .on_action(Message::ConfigFileEditorAction)
                ]
                .spacing(20)
                .align_x(Horizontal::Center);

                container(column)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Top)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            },
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

fn main() -> iced::Result {
    iced::application("Wynnbuilder Tools UI", Tabs::update, Tabs::view)
        .theme(|app: &Tabs| app.theme.clone())
        .run_with(Tabs::new)
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
    config_names
        .iter()
        .any(|name| Path::new("config").join(name).exists())
}
