use std::io::Read;
use std::path::Path;
use std::process::Stdio;

use config_file::ConfigFile;
use futures::channel::mpsc;
use futures::{SinkExt, Stream};
use iced::alignment::{Horizontal, Vertical};
use iced::stream::try_channel;
use iced::widget::container;
use iced::{Element, Length, Renderer, Task, Theme};
use iced_widget::text_editor::Action;
use iced_widget::{button, column, combo_box, pick_list, row, text, text_editor, Container};
use intro::Intro;
use messages::*;
use search_items::SearchItems;
use serde::{Deserialize, Serialize};

mod build_config;
mod config_file;
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
}

#[derive(Default)]
struct Tabs {
    active_tab: Tab,
    theme: Theme,
    intro_tab: Intro,
    search_items_tab: SearchItems,
    config_file_tab: ConfigFile,
    builder_tab: Builder,
}

#[derive(Default)]
struct Builder {
    state: State,
    builder_editor: text_editor::Content,
}

impl Builder {
    fn update(&mut self, message: BuilderMessage) -> Task<Message> {
        match message {
            BuilderMessage::Content(content) => {
                self.builder_editor = text_editor::Content::with_text(&content);
                Task::none()
            }
            BuilderMessage::Error(error) => {
                self.builder_editor = text_editor::Content::with_text(&error);
                Task::none()
            }
            BuilderMessage::StartBinary => {
                let state = State::new();
                self.state = state;

                Task::run(self.state.start_binary(), |result| match result {
                    Ok(content) => Message::Builder(BuilderMessage::Content(content)),
                    Err(error) => Message::Builder(BuilderMessage::Error(error)),
                })
            }
            BuilderMessage::Editor(action) => {
                match action {
                    Action::Edit(_) => (),
                    Action::Move(motion) => {
                        self.builder_editor.perform(Action::Move(motion));
                    },
                    Action::Select(motion) => {
                        self.builder_editor.perform(Action::Select(motion));
                    }
                    Action::SelectWord => {
                        self.builder_editor.perform(Action::SelectWord);
                    }
                    Action::SelectLine => {
                        self.builder_editor.perform(Action::SelectLine);
                    }
                    Action::SelectAll => {
                        self.builder_editor.perform(Action::SelectAll);
                    }
                    Action::Click(point) => {
                        self.builder_editor.perform(Action::Click(point));
                    }
                    Action::Drag(point) => {
                        self.builder_editor.perform(Action::Drag(point));
                    }
                    Action::Scroll { lines } => {
                        self.builder_editor.perform(Action::Scroll { lines });
                    }
                };
                Task::none()
            }
        }
    }

    fn view(&self) -> Container<Message> {
        let column = column![
            text("Builder").size(30),
            text("This tab is where the builder binary is run and monitored.").size(20),
            if self.state.is_running {
                Element::new(text("The builder is currently running.").size(20))
            } else {
                button("Start Builder")
                    .on_press(Message::Builder(BuilderMessage::StartBinary))
                    .into()
            },
            text_editor(&self.builder_editor)
                .on_action(|action| Message::Builder(BuilderMessage::Editor(action)))
        ];

        container(column)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Top)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

struct State {
    is_running: bool,
}

impl Default for State {
    fn default() -> Self {
        Self { is_running: false }
    }
}

impl State {
    pub fn new() -> Self {
        let instance = Self {
            ..Default::default()
        };

        instance
    }

    pub fn start_binary(&mut self) -> impl Stream<Item = Result<String, String>> {
        async fn match_send(
            output: &mut mpsc::Sender<String>,
            result: String,
        ) -> Result<(), String> {
            match output.send(result).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to send result: {}", e)),
            }
        }

        self.is_running = true;

        let binary_name = if cfg!(target_os = "windows") {
            "builder.exe"
        } else {
            "builder"
        };

        try_channel(1, move |mut output| async move {
            match_send(&mut output, "starting binary".to_string()).await?;

            let process_result = std::process::Command::new(binary_name)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn();

            let mut process = match process_result {
                Ok(process) => process,
                Err(e) => return Err(format!("Failed to start binary: {}", e)),
            };

            let mut process_reader = match process.stdout.take() {
                Some(reader) => reader,
                None => return Err("Failed to capture stdout".to_string()),
            };

            loop {
                let mut result = String::new();
                let _reader_future = process_reader.read_to_string(&mut result);

                if result.contains("done") {
                    match_send(&mut output, result).await?;
                    break;
                } else {
                    match_send(&mut output, result).await?;
                }
            }

            Ok(())
        })
    }
}

#[derive(Default)]
struct GearSelections {
    helmets: combo_box::State<String>,
    helmet_selections: Vec<Option<String>>,
    chestplates: combo_box::State<String>,
    chestplate_selections: Vec<Option<String>>,
    leggings: combo_box::State<String>,
    leggings_selections: Vec<Option<String>>,
    boots: combo_box::State<String>,
    boots_selections: Vec<Option<String>>,
    rings: combo_box::State<String>,
    rings_selections: Vec<Option<String>>,
    bracelets: combo_box::State<String>,
    bracelets_selections: Vec<Option<String>>,
    necklaces: combo_box::State<String>,
    necklaces_selections: Vec<Option<String>>,
    weapons: combo_box::State<String>,
    selected_weapon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GearList {
    items: Vec<Gear>,
}

impl GearList {
    fn from_json(path: &str) -> Result<Self, String> {
        let items_json_string = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read items file: {}", e))?;

        match serde_json::from_str::<GearList>(&items_json_string) {
            Ok(gear_list) => Ok(gear_list),
            Err(e) => {
                // Print more detailed error information
                eprintln!("Deserialization error: {}", e);
                Err(format!("Failed to parse items JSON: {}", e))
            }
        }
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

#[derive(Debug, Serialize, Deserialize, Default)]
struct Gear {
    id: i64,
    name: String,
    tier: String,
    #[serde(rename = "type")]
    gear_type: GearType,
}

impl Gear {
    fn default_for_type(gear_type: GearType, name: &str, id: i64) -> Self {
        Self {
            id,
            name: name.to_string(),       // Convert &str to owned String
            tier: String::from("Common"), // Default tier
            gear_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
enum GearType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Ring,
    Bracelet,
    Necklace,
    Bow,
    Spear,
    Wand,
    Dagger,
    Relik,
    #[default]
    None,
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

        // === Theme Setup ===
        let theme = match std::fs::read_to_string(&theme_path) {
            Ok(contents) => match toml::from_str::<ThemeConfig>(&contents) {
                Ok(config) => config.theme,
                Err(_) => Theme::Dark,
            },
            Err(_) => Theme::Dark,
        };

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
                theme,
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
            Tab::Intro => self.intro_tab.view(),
            Tab::Search => self.search_items_tab.view(),
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
            Tab::ConfigFile => self.config_file_tab.view(),
            Tab::Builder => self.builder_tab.view(),
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

fn gear_to_some(gear_list: Vec<String>) -> Vec<Option<String>> {
    let mut selections = vec![None; gear_list.len()];
    for (i, helmet) in gear_list.iter().enumerate() {
        selections[i] = Some(helmet.clone());
    }
    selections
}
