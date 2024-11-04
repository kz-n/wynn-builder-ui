use std::path::Path;

use build_config::Config;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_input};
use iced::{Element, Length, Renderer, Task, Theme};
use iced_widget::text_editor::Action;
use iced_widget::{combo_box, scrollable, text_editor, Container};
use messages::*;
use serde::{Deserialize, Serialize};

mod build_config;
mod messages;
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
    search_items_tab: SearchItemsTab,
    config_file_tab: ConfigFileTab,
    builder_tab: BuilderTab,
}

#[derive(Default)]
struct BuilderTab {
    builder_results: String,
    builder_editor: text_editor::Content,
    is_running: bool,
}

impl ConfigFileTab {
    fn save_config(&mut self) {
        self.config
            .save_config("config/config.toml")
            .unwrap_or_default();
    }
}

#[derive(Default)]
struct SearchItemsTab {
    search_input: String,
    search_results: text_editor::Content,
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

#[derive(Default)]
struct ConfigFileTab {
    error_message: Option<String>,
    gear: GearSelections,
    config: Config,
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
                config_file_tab: ConfigFileTab {
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
                search_items_tab: SearchItemsTab::default(),
                builder_tab: BuilderTab {
                    builder_results: String::new(),
                    builder_editor: text_editor::Content::with_text(&String::new()),
                    is_running: false,
                },
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
            Message::Search(search_message) => match search_message {
                SearchMessage::InputChanged(input_text) => {
                    self.search_items_tab.search_input = input_text;
                }
                SearchMessage::InputSubmitted => {
                    let binary_name = if cfg!(windows) {
                        "search_item.exe"
                    } else {
                        "search_item"
                    };

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
                SearchMessage::ItemEditorAction(action) => match action {
                    Action::Edit(_) => (),
                    Action::Move(_) => (),
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
                },
            },
            Message::Gear(gear_message) => match gear_message {
                GearMessage::HelmetSelected(idx, name) => {
                    if let Some(selection) =
                        self.config_file_tab.gear.helmet_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config_file_tab.config.items.helmets.push(name);
                    self.config_file_tab.save_config();
                }
                GearMessage::AddHelmet => {
                    self.config_file_tab.gear.helmet_selections.push(None);
                }
                GearMessage::ChestplateSelected(idx, name) => {
                    if let Some(selection) =
                        self.config_file_tab.gear.chestplate_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config_file_tab.config.items.chest_plates.push(name);
                    self.config_file_tab.save_config();
                }
                GearMessage::AddChestplate => {
                    self.config_file_tab.gear.chestplate_selections.push(None);
                }
                GearMessage::LeggingsSelected(idx, name) => {
                    if let Some(selection) =
                        self.config_file_tab.gear.leggings_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config_file_tab.config.items.leggings.push(name);
                    self.config_file_tab.save_config();
                }
                GearMessage::AddLeggings => {
                    self.config_file_tab.gear.leggings_selections.push(None);
                }
                GearMessage::BootsSelected(idx, name) => {
                    if let Some(selection) = self.config_file_tab.gear.boots_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config_file_tab.config.items.boots.push(name);
                    self.config_file_tab.save_config();
                }
                GearMessage::AddBoots => {
                    self.config_file_tab.gear.boots_selections.push(None);
                }
                GearMessage::RingsSelected(idx, name) => {
                    if let Some(selection) = self.config_file_tab.gear.rings_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config_file_tab.config.items.rings.push(name);
                    self.config_file_tab.save_config();
                }
                GearMessage::AddRings => {
                    self.config_file_tab.gear.rings_selections.push(None);
                }
                GearMessage::BraceletsSelected(idx, name) => {
                    if let Some(selection) =
                        self.config_file_tab.gear.bracelets_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config_file_tab.config.items.bracelets.push(name);
                    self.config_file_tab.save_config();
                }
                GearMessage::AddBracelets => {
                    self.config_file_tab.gear.bracelets_selections.push(None);
                }
                GearMessage::NecklacesSelected(idx, name) => {
                    if let Some(selection) =
                        self.config_file_tab.gear.necklaces_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config_file_tab.config.items.necklaces.push(name);
                    self.config_file_tab.save_config();
                }
                GearMessage::AddNecklaces => {
                    self.config_file_tab.gear.necklaces_selections.push(None);
                }
                GearMessage::WeaponSelected(name) => {
                    self.config_file_tab.gear.selected_weapon = Some(name);
                    self.config_file_tab.save_config();
                }
                GearMessage::RemoveHelmet(idx) => {
                    self.config_file_tab.gear.helmet_selections.remove(idx);
                    let helmet = self.config_file_tab.config.items.helmets.get(idx).cloned();
                    if let Some(helmet) = helmet {
                        self.config_file_tab
                            .config
                            .items
                            .helmets
                            .retain(|x| x != &helmet);
                    }
                    self.config_file_tab.save_config();
                }
                GearMessage::RemoveChestplate(idx) => {
                    self.config_file_tab.gear.chestplate_selections.remove(idx);
                    let chestplate = self
                        .config_file_tab
                        .config
                        .items
                        .chest_plates
                        .get(idx)
                        .cloned();
                    if let Some(chestplate) = chestplate {
                        self.config_file_tab
                            .config
                            .items
                            .chest_plates
                            .retain(|x| x != &chestplate);
                    }
                    self.config_file_tab.save_config();
                }
                GearMessage::RemoveLeggings(idx) => {
                    self.config_file_tab.gear.leggings_selections.remove(idx);
                    let legging = self.config_file_tab.config.items.leggings.get(idx).cloned();
                    if let Some(legging) = legging {
                        self.config_file_tab
                            .config
                            .items
                            .leggings
                            .retain(|x| x != &legging);
                    }
                    self.config_file_tab.save_config();
                }
                GearMessage::RemoveBoots(idx) => {
                    self.config_file_tab.gear.boots_selections.remove(idx);
                    let boot = self.config_file_tab.config.items.boots.get(idx).cloned();
                    if let Some(boot) = boot {
                        self.config_file_tab
                            .config
                            .items
                            .boots
                            .retain(|x| x != &boot);
                    }
                    self.config_file_tab.save_config();
                }
                GearMessage::RemoveRings(idx) => {
                    self.config_file_tab.gear.rings_selections.remove(idx);
                    let ring = self.config_file_tab.config.items.rings.get(idx).cloned();
                    if let Some(ring) = ring {
                        self.config_file_tab
                            .config
                            .items
                            .rings
                            .retain(|x| x != &ring);
                    }
                    self.config_file_tab.save_config();
                }
                GearMessage::RemoveBracelets(idx) => {
                    self.config_file_tab.gear.bracelets_selections.remove(idx);
                    let bracelet = self
                        .config_file_tab
                        .config
                        .items
                        .bracelets
                        .get(idx)
                        .cloned();
                    if let Some(bracelet) = bracelet {
                        self.config_file_tab
                            .config
                            .items
                            .bracelets
                            .retain(|x| x != &bracelet);
                    }
                    self.config_file_tab.save_config();
                }
                GearMessage::RemoveNecklaces(idx) => {
                    self.config_file_tab.gear.necklaces_selections.remove(idx);
                    let necklace = self
                        .config_file_tab
                        .config
                        .items
                        .necklaces
                        .get(idx)
                        .cloned();
                    if let Some(necklace) = necklace {
                        self.config_file_tab
                            .config
                            .items
                            .necklaces
                            .retain(|x| x != &necklace);
                    }
                    self.config_file_tab.save_config();
                }
            },
            Message::Player(player_message) => match player_message {
                PlayerMessage::LevelChanged(content) => {
                    if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config_file_tab.config.player.lvl = content.parse().unwrap_or(1);
                        self.config_file_tab.save_config();
                    }
                }
                PlayerMessage::AvailablePointChanged(content) => {
                    if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config_file_tab.config.player.available_point =
                            content.parse().unwrap_or(200);
                        self.config_file_tab.save_config();
                    }
                }
                PlayerMessage::BaseHpChanged(content) => {
                    if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config_file_tab.config.player.base_hp = content.parse().unwrap_or(500);
                        self.config_file_tab.save_config();
                    }
                }
            },
            Message::ThresholdFirst(threshold_first_message) => match threshold_first_message {
                ThresholdFirstMessage::HpChanged(content) => {
                    if content.is_empty() {
                        self.config_file_tab
                            .config
                            .threshold_first
                            .as_mut()
                            .unwrap()
                            .min_hp = None;
                    } else if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config_file_tab
                            .config
                            .threshold_first
                            .as_mut()
                            .unwrap()
                            .min_hp = Some(content.parse().unwrap_or(0));
                    }
                    self.config_file_tab.save_config();
                }
            },
            Message::ThresholdSecond(threshold_second_message) => {
                if let Some(threshold) = self.config_file_tab.config.threshold_second.as_mut() {
                    match threshold_second_message {
                        ThresholdSecondMessage::HprRawChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_hpr_raw = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::HprPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_hpr_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::MrChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_mr = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::LsChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_ls = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::MsChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_ms = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::SpdChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_spd = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::SdRawChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_sd_raw = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::SdPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_sd_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::HprChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_hpr = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdSecondMessage::ExpBonusChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_exp_bonus = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                    }
                    self.config_file_tab.save_config();
                }
            }
            Message::ThresholdThird(threshold_third_message) => {
                if let Some(threshold) = self.config_file_tab.config.threshold_third.as_mut() {
                    match threshold_third_message {
                        ThresholdThirdMessage::EarthDefChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_earth_defense = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdThirdMessage::ThunderDefChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_thunder_defense = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdThirdMessage::WaterDefChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_water_defense = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdThirdMessage::FireDefChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_fire_defense = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdThirdMessage::AirDefChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_air_defense = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                    }
                    self.config_file_tab.save_config();
                }
            }
            Message::ThresholdFourth(threshold_fourth_message) => {
                if let Some(threshold) = self.config_file_tab.config.threshold_fourth.as_mut() {
                    match threshold_fourth_message {
                        ThresholdFourthMessage::NeutralDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_neutral_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFourthMessage::EarthDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_earth_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFourthMessage::ThunderDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_thunder_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFourthMessage::WaterDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_water_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFourthMessage::FireDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_fire_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFourthMessage::AirDamPctChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_air_dam_pct = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                    }
                    self.config_file_tab.save_config();
                }
            }
            Message::ThresholdFifth(threshold_fifth_message) => {
                if let Some(threshold) = self.config_file_tab.config.threshold_fifth.as_mut() {
                    match threshold_fifth_message {
                        ThresholdFifthMessage::EarthPointChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_earth_point = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFifthMessage::ThunderPointChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_thunder_point = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFifthMessage::WaterPointChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_water_point = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFifthMessage::FirePointChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_fire_point = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFifthMessage::AirPointChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_air_point = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                        ThresholdFifthMessage::EhpChanged(content) => {
                            if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                                threshold.min_ehp = match content.is_empty() {
                                    true => None,
                                    false => Some(content.parse().unwrap_or(0)),
                                };
                            }
                        }
                    }
                    self.config_file_tab.save_config();
                }
            }
            Message::Hppeng(hppeng_message) => {
                match hppeng_message {
                    HppengMessage::UrlPrefixChanged(content) => {
                        self.config_file_tab.config.hppeng.url_prefix = content;
                    }
                    HppengMessage::UrlSuffixChanged(content) => {
                        self.config_file_tab.config.hppeng.url_suffix = content;
                    }
                    HppengMessage::DbPathChanged(content) => {
                        self.config_file_tab.config.hppeng.db_path = content;
                    }
                    HppengMessage::MigrationsPathChanged(content) => {
                        self.config_file_tab.config.hppeng.migrations_path = content;
                    }
                    HppengMessage::ItemsFileChanged(content) => {
                        self.config_file_tab.config.hppeng.items_file = content;
                    }
                    HppengMessage::LogBuildsChanged(value) => {
                        self.config_file_tab.config.hppeng.log_builds = value;
                    }
                    HppengMessage::LogDbErrorsChanged(value) => {
                        self.config_file_tab.config.hppeng.log_db_errors = value;
                    }
                    HppengMessage::DbRetryCountChanged(content) => {
                        self.config_file_tab.config.hppeng.db_retry_count =
                            content.parse().unwrap_or(3);
                    }
                }
                self.config_file_tab.save_config();
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
                    row![
                        checkbox(
                            "Items.json file found",
                            is_items_json_found(),
                        ).spacing(10),
                    ]
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
                    .on_input(|input| Message::Search(SearchMessage::InputChanged(input)))
                    .on_submit(Message::Search(SearchMessage::InputSubmitted))
                    .padding(10)
                    .width(Length::Fill),
                    text_editor(
                        &self.search_items_tab.search_results
                    )
                    .placeholder("Output will appear here...")
                    .on_action(|action| Message::Search(
                        SearchMessage::ItemEditorAction(action)
                    ))
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
                let content = column![
                    text("Edit Configuration File").size(30),
                    text("The configuration will be saved automatically when you edit.").size(20),
                    text("If the items say \"No items found\", check that items.json is present in the config folder, and close and re-open the application.").size(20),
                    // Error if no items.json file is found
                    if let Some(error_message) = &self.config_file_tab.error_message {
                        text(error_message).size(20).color(iced::color!(255, 0, 0))
                    } else {
                        text("").size(20)
                    },
                    text("Using this tab when there's a red error message above is undocumented and unexpected behavior, here be dragons!").size(16).color(iced::color!(255, 0, 0)),
                    text("Player Settings").size(20),
                    // Player settings
                    container(
                        column![
                            row![
                                text("Player Level:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter level (1-106)...",
                                    &self.config_file_tab.config.player.lvl.to_string()
                                )
                                .on_input(|input| Message::Player(PlayerMessage::LevelChanged(input)))
                                .size(16)
                                .padding(5)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Available Points:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter points...", 
                                    &self.config_file_tab.config.player.available_point.to_string()
                                )
                                .on_input(|input| Message::Player(PlayerMessage::AvailablePointChanged(input)))
                                .size(16)
                                .padding(5)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Base HP:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter base HP...",
                                    &self.config_file_tab.config.player.base_hp.to_string()
                                )
                                .on_input(|input| Message::Player(PlayerMessage::BaseHpChanged(input)))
                                .size(16)
                                .padding(5)
                                .width(Length::Fill),
                            ],
                        ]
                    )
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(10),
                    // Hppeng settings
                    text("Hppeng Settings").size(20),
                    container(
                        column![
                            row![
                                text("URL Prefix:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter URL prefix...",
                                    &self.config_file_tab.config.hppeng.url_prefix
                                )
                                .on_input(|input| Message::Hppeng(HppengMessage::UrlPrefixChanged(input)))
                                .size(16)
                                .padding(5)
                                .width(Length::Fill),
                            ],
                            row![
                                text("URL Suffix:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter URL suffix...",
                                    &self.config_file_tab.config.hppeng.url_suffix
                                )
                                .on_input(|input| Message::Hppeng(HppengMessage::UrlSuffixChanged(input)))
                                .size(16)
                                .padding(5)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Database Path:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter database path...",
                                    &self.config_file_tab.config.hppeng.db_path
                                )
                                .on_input(|input| Message::Hppeng(HppengMessage::DbPathChanged(input)))
                                .size(16)
                                .padding(5)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Migrations Path:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter migrations path...",
                                    &self.config_file_tab.config.hppeng.migrations_path
                                )
                                .on_input(|input| Message::Hppeng(HppengMessage::MigrationsPathChanged(input)))
                                .size(16)
                                .padding(5)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Items File:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter items file path...",
                                    &self.config_file_tab.config.hppeng.items_file
                                )
                                .on_input(|input| Message::Hppeng(HppengMessage::ItemsFileChanged(input)))
                                .size(16)
                                .padding(5)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Log Builds:").width(Length::Fixed(150.0)),
                                checkbox(
                                    "",
                                    self.config_file_tab.config.hppeng.log_builds,
                                )
                                .on_toggle(|value| Message::Hppeng(HppengMessage::LogBuildsChanged(value)))
                                .size(16)
                                .spacing(5),
                            ],
                            row![
                                text("Log DB Errors:").width(Length::Fixed(150.0)),
                                checkbox(
                                    "",
                                    self.config_file_tab.config.hppeng.log_db_errors,
                                )
                                .on_toggle(|value| Message::Hppeng(HppengMessage::LogDbErrorsChanged(value)))
                                .size(16)
                                .spacing(5),
                            ],
                            row![
                                text("DB Retry Count:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter retry count...",
                                    &self.config_file_tab.config.hppeng.db_retry_count.to_string()
                                )
                                .on_input(|input| Message::Hppeng(HppengMessage::DbRetryCountChanged(input)))
                                .size(16)
                                .padding(5)
                                .width(Length::Fill),
                            ],
                        ]
                    )
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(10),
                    // Threshold First settings
                    text("Threshold First Settings").size(20),
                    container(
                        column![
                            row![
                        text("Min HP:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min HP...",
                            &self.config_file_tab.config.threshold_first.as_ref().unwrap().min_hp
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdFirst(ThresholdFirstMessage::HpChanged(input)))
                        .padding(5)
                        .width(Length::Fill),
                            ],
                        ]
                    )
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(10),
                    // Threshold Second settings
                    text("Threshold Second Settings").size(20),
                    container(
                        column![
                            row![
                        text("Min HPR Raw:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min HPR raw...",
                            &self.config_file_tab.config.threshold_second.as_ref().unwrap().min_hpr_raw
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdSecond(ThresholdSecondMessage::HprRawChanged(input)))
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min HPR Percentage:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min HPR percentage...",
                            &self.config_file_tab.config.threshold_second.as_ref().unwrap().min_hpr_pct
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdSecond(ThresholdSecondMessage::HprPctChanged(input)))
                        .size(16)
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min MR:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min MR...",
                            &self.config_file_tab.config.threshold_second.as_ref().unwrap().min_mr
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdSecond(ThresholdSecondMessage::MrChanged(input)))
                        .size(16)
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min LS:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min LS...",
                            &self.config_file_tab.config.threshold_second.as_ref().unwrap().min_ls
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdSecond(ThresholdSecondMessage::LsChanged(input)))
                        .size(16)
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min MS:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min MS...",
                            &self.config_file_tab.config.threshold_second.as_ref().unwrap().min_ms
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdSecond(ThresholdSecondMessage::MsChanged(input)))
                        .size(16)
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min SPD:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min SPD...",
                            &self.config_file_tab.config.threshold_second.as_ref().unwrap().min_spd
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdSecond(ThresholdSecondMessage::SpdChanged(input)))
                        .size(16)
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min SD Raw:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min SD raw...",
                            &self.config_file_tab.config.threshold_second.as_ref().unwrap().min_sd_raw
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdSecond(ThresholdSecondMessage::SdRawChanged(input)))
                        .size(16)
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min SD Percentage:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min SD percentage...",
                            &self.config_file_tab.config.threshold_second.as_ref().unwrap().min_sd_pct
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdSecond(ThresholdSecondMessage::SdPctChanged(input)))
                        .size(16)
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min HPR:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min HPR...",
                            &self.config_file_tab.config.threshold_second.as_ref().unwrap().min_hpr
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdSecond(ThresholdSecondMessage::HprChanged(input)))
                        .size(16)
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min EXP Bonus:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min EXP bonus...",
                            &self.config_file_tab.config.threshold_second.as_ref().unwrap().min_exp_bonus
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdSecond(ThresholdSecondMessage::ExpBonusChanged(input)))
                        .size(16)
                        .padding(5)
                        .width(Length::Fill),
                            ],
                        ]
                    )
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(10),
                    // Threshold Third settings
                    text("Threshold Third Settings").size(20),
                    container(
                        column![
                            row![
                        text("Min Earth Defense:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min earth defense...",
                            &self.config_file_tab.config.threshold_third.as_ref().unwrap().min_earth_defense
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdThird(ThresholdThirdMessage::EarthDefChanged(input)))
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min Thunder Defense:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min thunder defense...",
                            &self.config_file_tab.config.threshold_third.as_ref().unwrap().min_thunder_defense
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdThird(ThresholdThirdMessage::ThunderDefChanged(input)))
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min Water Defense:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min water defense...",
                            &self.config_file_tab.config.threshold_third.as_ref().unwrap().min_water_defense
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdThird(ThresholdThirdMessage::WaterDefChanged(input)))
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min Fire Defense:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min fire defense...",
                            &self.config_file_tab.config.threshold_third.as_ref().unwrap().min_fire_defense
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdThird(ThresholdThirdMessage::FireDefChanged(input)))
                        .padding(5)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min Air Defense:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min air defense...",
                            &self.config_file_tab.config.threshold_third.as_ref().unwrap().min_air_defense
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdThird(ThresholdThirdMessage::AirDefChanged(input)))
                        .padding(5)
                        .width(Length::Fill),
                    ],
                        ]
                    )
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(10),
                    // Threshold Fourth settings
                    text("Threshold Fourth Settings").size(20),
                    container(
                        column![
                            row![
                        text("Min Neutral Damage %:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min neutral damage %...",
                            &self.config_file_tab.config.threshold_fourth.as_ref().unwrap().min_neutral_dam_pct
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdFourth(ThresholdFourthMessage::NeutralDamPctChanged(input)))
                        .padding(10)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min Earth Damage %:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min earth damage %...",
                            &self.config_file_tab.config.threshold_fourth.as_ref().unwrap().min_earth_dam_pct
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdFourth(ThresholdFourthMessage::EarthDamPctChanged(input)))
                        .padding(10)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min Thunder Damage %:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min thunder damage %...",
                            &self.config_file_tab.config.threshold_fourth.as_ref().unwrap().min_thunder_dam_pct
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdFourth(ThresholdFourthMessage::ThunderDamPctChanged(input)))
                        .padding(10)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min Water Damage %:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min water damage %...",
                            &self.config_file_tab.config.threshold_fourth.as_ref().unwrap().min_water_dam_pct
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdFourth(ThresholdFourthMessage::WaterDamPctChanged(input)))
                        .padding(10)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min Fire Damage %:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min fire damage %...",
                            &self.config_file_tab.config.threshold_fourth.as_ref().unwrap().min_fire_dam_pct
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdFourth(ThresholdFourthMessage::FireDamPctChanged(input)))
                        .padding(10)
                        .width(Length::Fill),
                    ],
                    row![
                        text("Min Air Damage %:").width(Length::Fixed(150.0)),
                        text_input(
                            "Enter min air damage %...",
                            &self.config_file_tab.config.threshold_fourth.as_ref().unwrap().min_air_dam_pct
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        )
                        .on_input(|input| Message::ThresholdFourth(ThresholdFourthMessage::AirDamPctChanged(input)))
                        .padding(10)
                        .width(Length::Fill),
                    ],
                        ]
                    ),
                    // Threshold Fifth settings
                    text("Threshold Fifth Settings").size(20),
                    container(
                        column![
                            row![
                                text("Min Earth Point:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter min earth point...",
                                    &self.config_file_tab.config.threshold_fifth.as_ref().unwrap().min_earth_point
                                        .map(|v| v.to_string())
                                        .unwrap_or_default()
                                )
                                .on_input(|input| Message::ThresholdFifth(ThresholdFifthMessage::EarthPointChanged(input)))
                                .padding(10)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Min Thunder Point:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter min thunder point...",
                                    &self.config_file_tab.config.threshold_fifth.as_ref().unwrap().min_thunder_point
                                        .map(|v| v.to_string())
                                        .unwrap_or_default()
                                )
                                .on_input(|input| Message::ThresholdFifth(ThresholdFifthMessage::ThunderPointChanged(input)))
                                .padding(10)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Min Water Point:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter min water point...",
                                    &self.config_file_tab.config.threshold_fifth.as_ref().unwrap().min_water_point
                                        .map(|v| v.to_string())
                                        .unwrap_or_default()
                                )
                                .on_input(|input| Message::ThresholdFifth(ThresholdFifthMessage::WaterPointChanged(input)))
                                .padding(10)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Min Fire Point:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter min fire point...",
                                    &self.config_file_tab.config.threshold_fifth.as_ref().unwrap().min_fire_point
                                        .map(|v| v.to_string())
                                        .unwrap_or_default()
                                )
                                .on_input(|input| Message::ThresholdFifth(ThresholdFifthMessage::FirePointChanged(input)))
                                .padding(10)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Min Air Point:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter min air point...",
                                    &self.config_file_tab.config.threshold_fifth.as_ref().unwrap().min_air_point
                                        .map(|v| v.to_string())
                                        .unwrap_or_default()
                                )
                                .on_input(|input| Message::ThresholdFifth(ThresholdFifthMessage::AirPointChanged(input)))
                                .padding(10)
                                .width(Length::Fill),
                            ],
                            row![
                                text("Min EHP:").width(Length::Fixed(150.0)),
                                text_input(
                                    "Enter min EHP...",
                                    &self.config_file_tab.config.threshold_fifth.as_ref().unwrap().min_ehp
                                        .map(|v| v.to_string())
                                        .unwrap_or_default()
                                )
                                .on_input(|input| Message::ThresholdFifth(ThresholdFifthMessage::EhpChanged(input)))
                                .padding(10)
                                .width(Length::Fill),
                            ],
                        ]
                    )
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(10),
                    // Gear Selection - Helmets
                    column![
                        text("Helmets:"),
                        self.config_file_tab
                            .gear
                            .helmet_selections
                            .iter()
                            .enumerate()
                            .fold(column![].spacing(5), |col, (idx, selection)| {
                                col.push(
                                    row![
                                        combo_box(
                                            &self.config_file_tab.gear.helmets,
                                            "Select helmet...",
                                            selection.as_ref(),
                                            move |name| Message::Gear(GearMessage::HelmetSelected(idx, name)),
                                        ),
                                        button("X")
                                            .on_press(Message::Gear(GearMessage::RemoveHelmet(idx)))
                                            .padding(5),
                                    ]
                                    .spacing(10)
                                )
                            }),
                        button("Add Helmet").on_press(Message::Gear(GearMessage::AddHelmet)),
                    ]
                    .spacing(10),
                    // Gear Selection - Chestplates
                    column![
                        text("Chestplates:"),
                        self.config_file_tab
                            .gear
                            .chestplate_selections
                            .iter()
                            .enumerate()
                            .fold(column![].spacing(5), |col, (idx, selection)| {
                                col.push(
                                    row![
                                        combo_box(
                                            &self.config_file_tab.gear.chestplates,
                                            "Select chestplate...",
                                            selection.as_ref(),
                                            move |name| Message::Gear(GearMessage::ChestplateSelected(idx, name)),
                                        ),
                                        button("X")
                                            .on_press(Message::Gear(GearMessage::RemoveChestplate(idx)))
                                            .padding(5),
                                    ]
                                    .spacing(10)
                                )
                            }),
                        button("Add Chestplate").on_press(Message::Gear(GearMessage::AddChestplate)),
                    ]
                    .spacing(10),
                    // Gear Selection - Leggings
                    column![
                        text("Leggings:"),
                        self.config_file_tab
                            .gear
                            .leggings_selections
                            .iter()
                            .enumerate()
                            .fold(column![].spacing(5), |col, (idx, selection)| {
                                col.push(
                                    row![
                                        combo_box(
                                            &self.config_file_tab.gear.leggings,
                                            "Select leggings...",
                                            selection.as_ref(),
                                            move |name| Message::Gear(GearMessage::LeggingsSelected(idx, name)),
                                        ),
                                        button("X")
                                            .on_press(Message::Gear(GearMessage::RemoveLeggings(idx)))
                                            .padding(5),
                                    ]
                                    .spacing(10)
                                )
                            }),
                        button("Add Leggings").on_press(Message::Gear(GearMessage::AddLeggings)),
                    ]
                    .spacing(10),
                    // Gear Selection - Boots
                    column![
                        text("Boots:"),
                        self.config_file_tab
                            .gear
                            .boots_selections
                            .iter()
                            .enumerate()
                            .fold(column![].spacing(5), |col, (idx, selection)| {
                                col.push(
                                    row![
                                        combo_box(
                                            &self.config_file_tab.gear.boots,
                                            "Select boots...",
                                            selection.as_ref(),
                                            move |name| Message::Gear(GearMessage::BootsSelected(idx, name)),
                                        ),
                                        button("X")
                                            .on_press(Message::Gear(GearMessage::RemoveBoots(idx)))
                                            .padding(5),
                                    ]
                                    .spacing(10)
                                )
                            }),
                        button("Add Boots").on_press(Message::Gear(GearMessage::AddBoots)),
                    ]
                    .spacing(10),
                    // Gear Selection - Rings
                    column![
                        text("Rings:"),
                        self.config_file_tab
                            .gear
                            .rings_selections
                            .iter()
                            .enumerate()
                            .fold(column![].spacing(5), |col, (idx, selection)| {
                                col.push(
                                    row![
                                        combo_box(
                                            &self.config_file_tab.gear.rings,
                                            "Select ring...",
                                            selection.as_ref(),
                                            move |name| Message::Gear(GearMessage::RingsSelected(idx, name)),
                                        ),
                                        button("X")
                                            .on_press(Message::Gear(GearMessage::RemoveRings(idx)))
                                            .padding(5),
                                    ]
                                    .spacing(10)
                                )
                            }),
                        button("Add Ring").on_press(Message::Gear(GearMessage::AddRings)),
                    ]
                    .spacing(10),
                    // Gear Selection - Bracelets
                    column![
                        text("Bracelets:"),
                        self.config_file_tab
                            .gear
                            .bracelets_selections
                            .iter()
                            .enumerate()
                            .fold(column![].spacing(5), |col, (idx, selection)| {
                                col.push(
                                    row![
                                        combo_box(
                                            &self.config_file_tab.gear.bracelets,
                                            "Select bracelet...",
                                            selection.as_ref(),
                                            move |name| Message::Gear(GearMessage::BraceletsSelected(idx, name)),
                                        ),
                                        button("X")
                                            .on_press(Message::Gear(GearMessage::RemoveBracelets(idx)))
                                            .padding(5),
                                    ]
                                    .spacing(10)
                                )
                            }),
                        button("Add Bracelet").on_press(Message::Gear(GearMessage::AddBracelets)),
                    ]
                    .spacing(10),
                    // Gear Selection - Necklaces
                    column![
                        text("Necklaces:"),
                        self.config_file_tab
                            .gear
                            .necklaces_selections
                            .iter()
                            .enumerate()
                            .fold(column![].spacing(5), |col, (idx, selection)| {
                                col.push(
                                    row![
                                        combo_box(
                                            &self.config_file_tab.gear.necklaces,
                                            "Select necklace...",
                                            selection.as_ref(),
                                            move |name| Message::Gear(GearMessage::NecklacesSelected(idx, name)),
                                        ),
                                        button("X")
                                            .on_press(Message::Gear(GearMessage::RemoveNecklaces(idx)))
                                            .padding(5),
                                    ]
                                    .spacing(10)
                                )
                            }),
                        button("Add Necklace").on_press(Message::Gear(GearMessage::AddNecklaces)),
                    ]
                    .spacing(10),
                    // Gear Selection - Weapon
                    column![
                        text("Weapon:"),
                        combo_box(
                            &self.config_file_tab.gear.weapons,
                            "Select weapon...",
                            self.config_file_tab.gear.selected_weapon.as_ref(),
                            |name| Message::Gear(GearMessage::WeaponSelected(name)),
                        ),
                    ]
                    .spacing(10),
                ]
                .spacing(20)
                .align_x(Horizontal::Left);

                // Wrap the content in a container with padding before scrollable
                container(
                    scrollable(
                        container(content)
                            .padding(20) // Add padding around the content
                            .width(Length::Fill),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill),
                )
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

fn gear_to_some(gear_list: Vec<String>) -> Vec<Option<String>> {
    let mut selections = vec![None; gear_list.len()];
    for (i, helmet) in gear_list.iter().enumerate() {
        selections[i] = Some(helmet.clone());
    }
    selections
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

fn is_items_json_found() -> bool {
    Path::new("config/items.json").exists()
}
