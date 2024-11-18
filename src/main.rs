use builder::Builder;
use config::{ConfigFile, Gear, GearList, GearSelections, GearType};
use db_reader::DBReader;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::container;
use iced::{Element, Length, Renderer, Task, Theme};
use iced_widget::{button, column, combo_box, pick_list, row, text, Container};
use intro::Intro;
use messages::*;
use search_items::SearchItems;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod build_config;
mod builder;
mod config;
mod db_reader;
mod intro;
mod messages;
mod search_items;
mod theme_serde;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Tab {
    #[default]
    Intro,
    Search,
    ConfigFile,
    Builder,
    Theme,
    DBReader,
}

#[derive(Default)]
struct Tabs {
    active_tab: Tab,
    theme: Theme,
    intro_tab: Intro,
    search_items_tab: SearchItems,
    config_file_tab: ConfigFile,
    builder_tab: Builder,
    db_reader_tab: DBReader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThemeConfig {
    #[serde(with = "theme_serde")]
    theme: Theme,
}

impl Tabs {
    fn new() -> (Self, Task<Message>) {
        // === Directory Setup ===
        let settings_dir = Path::new("settings");
        let _ = std::fs::create_dir_all(settings_dir);
        let theme_path = settings_dir.join("theme.toml");

        // === Theme Setup ===
        let theme = match std::fs::read_to_string(&theme_path) {
            Ok(contents) => match toml::from_str::<ThemeConfig>(&contents) {
                Ok(config) => config.theme,
                Err(_) => Theme::Dark,
            },
            Err(_) => Theme::Dark,
        };

        // === Load Config File ===
        let config = build_config::load_config("config/config.toml").unwrap_or_default();

        // Parse config items into vectors
        let helmets = config.items.helmets.iter().map(|s| s.to_string()).collect();
        let chestplates = config
            .items
            .chest_plates
            .iter()
            .map(|s| s.to_string())
            .collect();
        let leggings = config
            .items
            .leggings
            .iter()
            .map(|s| s.to_string())
            .collect();
        let boots = config.items.boots.iter().map(|s| s.to_string()).collect();
        let rings = config.items.rings.iter().map(|s| s.to_string()).collect();
        let bracelets = config
            .items
            .bracelets
            .iter()
            .map(|s| s.to_string())
            .collect();
        let necklaces = config
            .items
            .necklaces
            .iter()
            .map(|s| s.to_string())
            .collect();
        let weapon = config.items.weapon.to_string();

        // === Load Gear List ===
        let (gear_list, error_message) = match GearList::from_json("config/items.json") {
            Ok(list) => (list, "ok".to_owned()),
            Err(e) => (
                GearList {
                    items: vec![
                        // Default gear items when loading fails
                        Gear::default_for_type(GearType::Helmet, "No helmets found", -1),
                        Gear::default_for_type(GearType::Chestplate, "No chestplates found", -2),
                        Gear::default_for_type(GearType::Leggings, "No leggings found", -3),
                        Gear::default_for_type(GearType::Boots, "No boots found", -4),
                        Gear::default_for_type(GearType::Ring, "No rings found", -5),
                        Gear::default_for_type(GearType::Bracelet, "No bracelets found", -6),
                        Gear::default_for_type(GearType::Necklace, "No necklaces found", -7),
                        Gear::default_for_type(GearType::Spear, "No weapons found", -8),
                    ],
                },
                format!("Error loading items.json: {}", e),
            ),
        };

        let selected_weapon = if weapon.is_empty() {
            None
        } else {
            Some(weapon)
        };

        // === Return Initialized State ===
        (
            Self {
                active_tab: Tab::Intro,
                theme: theme.clone(),
                // Config File Tab initialization
                config_file_tab: ConfigFile {
                    // Gear selection states
                    gear: GearSelections {
                        helmets: combo_box::State::new(gear_list.helmets()),
                        helmet_selections: gear_to_some(helmets),
                        chestplates: combo_box::State::new(gear_list.chestplates()),
                        chestplate_selections: gear_to_some(chestplates),
                        leggings: combo_box::State::new(gear_list.leggings()),
                        leggings_selections: gear_to_some(leggings),
                        boots: combo_box::State::new(gear_list.boots()),
                        boots_selections: gear_to_some(boots),
                        rings: combo_box::State::new(gear_list.rings()),
                        rings_selections: gear_to_some(rings),
                        bracelets: combo_box::State::new(gear_list.bracelets()),
                        bracelets_selections: gear_to_some(bracelets),
                        necklaces: combo_box::State::new(gear_list.necklaces()),
                        necklaces_selections: gear_to_some(necklaces),
                        weapons: combo_box::State::new(gear_list.weapons()),
                        selected_weapon,
                    },
                    // Error handling
                    error_message: if error_message != "ok" {
                        Some(error_message)
                    } else {
                        None
                    },
                    config,
                    ..Default::default()
                },
                // Search Tab initialization
                search_items_tab: SearchItems::default(),
                builder_tab: Builder::default(),
                db_reader_tab: DBReader::init(theme.clone()),
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                Task::none()
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
                Task::none()
            }
            Message::Search(search_message) => self.search_items_tab.update(search_message),
            Message::Config(config_message) => self.config_file_tab.update(config_message),
            Message::Builder(builder_message) => self.builder_tab.update(builder_message),
            Message::DBReader(dbreader_message) => self.db_reader_tab.update(dbreader_message),
        }
    }

    fn view(&self) -> Element<Message> {
        // Create tab buttons
        let tab_buttons = row![
            button("Intro").on_press(Message::TabSelected(Tab::Intro)),
            button("Search").on_press(Message::TabSelected(Tab::Search)),
            button("Config File").on_press(Message::TabSelected(Tab::ConfigFile)),
            button("Builder").on_press(Message::TabSelected(Tab::Builder)),
            button("DB Reader").on_press(Message::TabSelected(Tab::DBReader)),
            button("Theme").on_press(Message::TabSelected(Tab::Theme)),
        ]
        .spacing(4);

        // Create content based on active tab
        let content: Container<'_, Message, Theme, Renderer> = match self.active_tab {
            Tab::Intro => self.intro_tab.view(),
            Tab::Search => self.search_items_tab.view(),
            Tab::Theme => {
                let selector = column![
                    text("Select Theme:"),
                    pick_list(
                        Themes::to_vec(),
                        Some(self.theme.clone()),
                        Message::ThemeChanged,
                    )
                ];

                container(selector)
                    .align_x(Horizontal::Left)
                    .align_y(Vertical::Top)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            Tab::ConfigFile => self.config_file_tab.view(),
            Tab::Builder => self.builder_tab.view(),
            Tab::DBReader => self.db_reader_tab.view(),
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

macro_rules! define_themes {
    ($($variant:ident),*) => {
        pub enum Themes {
            $($variant),*
        }

        impl Themes {
            pub fn to_vec() -> Vec<Theme> {
                vec![
                    $(Theme::$variant),*
                ]
            }

            pub fn to_vec_str() -> Vec<String> {
                Self::to_vec().iter().map(|theme| theme.to_string()).collect()
            }

            pub fn from_str(theme: &str) -> Option<Self> {
                match theme {
                    $(stringify!($variant) => Some(Themes::$variant),)*
                    _ => None,
                }
            }
        }

        impl ToString for Themes {
            fn to_string(&self) -> String {
                match self {
                    $(Themes::$variant => stringify!($variant).to_string()),*
                }
            }
        }
    }
}

define_themes!(
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra
);

fn main() -> iced::Result {
    iced::application("Wynnbuilder Tools UI", Tabs::update, Tabs::view)
        .theme(|app: &Tabs| app.theme.clone())
        .run_with(Tabs::new)
}

fn gear_to_some(gear_list: Vec<String>) -> Vec<Option<String>> {
    let mut selections = vec![None; gear_list.len()];
    for (i, helmet) in gear_list.iter().enumerate() {
        selections[i] = Some(helmet.clone());
    }
    selections
}
