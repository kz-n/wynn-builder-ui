use std::path::Path;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_input};
use iced::{Element, Length, Renderer, Task, Theme};
use iced_widget::text_editor::Action;
use iced_widget::{combo_box, text_editor, Container};
use serde::{Deserialize, Serialize};

mod theme_serde;

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
    HelmetSelected(String),
    ChestplateSelected(String),
    LeggingsSelected(String),
    BootsSelected(String),
    RingsSelected(String),
    BraceletsSelected(String),
    NecklacesSelected(String),
    WeaponsSelected(String),
}

#[derive(Default)]
struct Tabs {
    active_tab: Tab,
    theme: Theme,
    search_items_tab: SearchItemsTab,
    config_file_tab: ConfigFileTab,
}

#[derive(Default)]
struct SearchItemsTab {
    search_input: String,
    search_results: text_editor::Content,
}

#[derive(Default)]
struct ConfigFileTab {
    helmets: combo_box::State<String>,
    selected_helmet: Option<String>,
    chestplates: combo_box::State<String>,
    selected_chestplate: Option<String>,
    leggings: combo_box::State<String>,
    selected_leggings: Option<String>,
    boots: combo_box::State<String>,
    selected_boots: Option<String>,
    rings: combo_box::State<String>,
    selected_rings: Option<String>,
    bracelets: combo_box::State<String>,
    selected_bracelets: Option<String>,
    necklaces: combo_box::State<String>,
    selected_necklaces: Option<String>,
    weapons: combo_box::State<String>,
    selected_weapon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GearList {
    items: Vec<Gear>,
}

impl GearList {
    fn from_json(path: &str) -> Self {
        let items_json_string = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&items_json_string).unwrap()
    }

    fn get_gear_by_type(&self, gear_type: GearType) -> Vec<String> {
        self.items
            .iter()
            .filter(|gear| gear.gear_type == gear_type)
            .map(|gear| gear.name.clone())
            .collect()
    }

    fn helmets(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Helmet)
    }

    fn chestplates(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Chestplate)
    }

    fn leggings(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Leggings)
    }

    fn boots(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Boots)
    }

    fn rings(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Ring)
    }

    fn bracelets(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Bracelet)
    }

    fn necklaces(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Necklace)
    }

    fn weapons(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Spear)
            .into_iter()
            .chain(self.get_gear_by_type(GearType::Wand))
            .chain(self.get_gear_by_type(GearType::Bow))
            .chain(self.get_gear_by_type(GearType::Dagger))
            .chain(self.get_gear_by_type(GearType::Relik))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Gear {
    id: i64,
    name: String,
    #[serde(rename = "type")]
    gear_type: GearType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum GearType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Ring,
    Bracelet,
    Necklace,
    Spear,
    Wand,
    Bow,
    Dagger,
    Relik,
}

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

        let gear_list = GearList::from_json("config/items.json");

        (
            Self {
                active_tab: Tab::Intro,
                theme,
                config_file_tab: ConfigFileTab {
                    helmets: combo_box::State::new(gear_list.helmets()),
                    chestplates: combo_box::State::new(gear_list.chestplates()),
                    leggings: combo_box::State::new(gear_list.leggings()),
                    boots: combo_box::State::new(gear_list.boots()),
                    rings: combo_box::State::new(gear_list.rings()),
                    bracelets: combo_box::State::new(gear_list.bracelets()),
                    necklaces: combo_box::State::new(gear_list.necklaces()),
                    weapons: combo_box::State::new(gear_list.weapons()),
                    ..Default::default()
                },
                search_items_tab: SearchItemsTab::default(),
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
                self.search_items_tab.search_input = input_text;
            }
            Message::SearchInputSubmitted => {
                let binary_name = if cfg!(windows) {
                    "search_item.exe"
                } else {
                    "search_item"
                };

                // Split input into arguments, preserving quoted strings if present
                let args: Vec<&str> = self
                    .search_items_tab
                    .search_input
                    .split_whitespace()
                    .collect();

                let output = match std::process::Command::new(binary_name).args(args).output() {
                    Ok(output) => match output.status.success() {
                        true => String::from_utf8_lossy(&output.stdout).to_string(),
                        false => String::from_utf8_lossy(&output.stderr).to_string(),
                    },
                    Err(e) => format!("Error: Could not execute search_item binary: {}", e),
                };

                self.search_items_tab.search_results = text_editor::Content::with_text(&output);
            }
            Message::Ignore => (),
            Message::SearchItemStdout(output) => {
                self.search_items_tab.search_results = text_editor::Content::with_text(&output);
            }
            Message::SearchItemEditorAction(action) => {
                match action {
                    Action::Edit(_) => (), // Do nothing for edits
                    Action::Move(_) => (), // Do nothing for moves
                    Action::Select(motion) => {
                        self.search_items_tab
                            .search_results
                            .perform(Action::Select(motion));
                    }
                    Action::SelectWord => {
                        self.search_items_tab
                            .search_results
                            .perform(Action::SelectWord);
                    }
                    Action::SelectLine => {
                        self.search_items_tab
                            .search_results
                            .perform(Action::SelectLine);
                    }
                    Action::SelectAll => {
                        self.search_items_tab
                            .search_results
                            .perform(Action::SelectAll);
                    }
                    Action::Click(point) => {
                        self.search_items_tab
                            .search_results
                            .perform(Action::Click(point));
                    }
                    Action::Drag(point) => {
                        self.search_items_tab
                            .search_results
                            .perform(Action::Drag(point));
                    }
                    Action::Scroll { lines } => {
                        self.search_items_tab
                            .search_results
                            .perform(Action::Scroll { lines });
                    }
                }
            }
            Message::HelmetSelected(name) => {
                self.config_file_tab.selected_helmet = Some(name);
            }
            Message::ChestplateSelected(name) => {
                self.config_file_tab.selected_chestplate = Some(name);
            }
            Message::LeggingsSelected(name) => {
                self.config_file_tab.selected_leggings = Some(name);
            }
            Message::BootsSelected(name) => {
                self.config_file_tab.selected_boots = Some(name);
            }
            Message::RingsSelected(name) => {
                self.config_file_tab.selected_rings = Some(name);
            }
            Message::BraceletsSelected(name) => {
                self.config_file_tab.selected_bracelets = Some(name);
            }
            Message::NecklacesSelected(name) => {
                self.config_file_tab.selected_necklaces = Some(name);
            }
            Message::WeaponsSelected(name) => {
                self.config_file_tab.selected_weapon = Some(name);
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
                        &self.search_items_tab.search_input
                    )
                    .on_input(|input| Message::SearchInputChanged(input))
                    .on_submit(Message::SearchInputSubmitted)
                    .padding(10)
                    .width(Length::Fill),
                    text_editor(
                        &self.search_items_tab.search_results
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
                    text("Edit Configuration File").size(30),
                    text("The configuration will be saved automatically when you edit.").size(20),
                    combo_box(
                        &self.config_file_tab.helmets,
                        "Select a helmet...",
                        self.config_file_tab.selected_helmet.as_ref(),
                        Message::HelmetSelected
                    ),
                    combo_box(
                        &self.config_file_tab.chestplates,
                        "Select a chestplate...",
                        self.config_file_tab.selected_chestplate.as_ref(),
                        Message::ChestplateSelected
                    ),
                    combo_box(
                        &self.config_file_tab.leggings,
                        "Select a leggings...",
                        self.config_file_tab.selected_leggings.as_ref(),
                        Message::LeggingsSelected
                    ),
                    combo_box(
                        &self.config_file_tab.boots,
                        "Select a boots...",
                        self.config_file_tab.selected_boots.as_ref(),
                        Message::BootsSelected
                    ),
                    combo_box(
                        &self.config_file_tab.rings,
                        "Select a ring...",
                        self.config_file_tab.selected_rings.as_ref(),
                        Message::RingsSelected
                    ),
                    combo_box(
                        &self.config_file_tab.bracelets,
                        "Select a bracelet...",
                        self.config_file_tab.selected_bracelets.as_ref(),
                        Message::BraceletsSelected
                    ),
                    combo_box(
                        &self.config_file_tab.necklaces,
                        "Select a necklace...",
                        self.config_file_tab.selected_necklaces.as_ref(),
                        Message::NecklacesSelected
                    ),
                    combo_box(
                        &self.config_file_tab.weapons,
                        "Select a weapon...",
                        self.config_file_tab.selected_weapon.as_ref(),
                        Message::WeaponsSelected
                    ),
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
