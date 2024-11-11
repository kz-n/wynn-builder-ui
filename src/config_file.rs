use iced::{
    alignment::{Horizontal, Vertical},
    Length, Task,
};
use iced_widget::{
    button, checkbox, column, combo_box, container, row, scrollable, text, text_input, Container,
};
use serde::{Deserialize, Serialize};

use crate::{
    build_config::Config, ConfigMessage, GearMessage, HppengMessage, Message, PlayerMessage, ThresholdFifthMessage, ThresholdFirstMessage, ThresholdFourthMessage, ThresholdSecondMessage, ThresholdThirdMessage
};

#[derive(Default)]
pub struct ConfigFile {
    pub error_message: Option<String>,
    pub gear: GearSelections,
    pub config: Config,
}

impl ConfigFile {
    pub fn update(&mut self, message: ConfigMessage) -> Task<Message> {
        match message {
            ConfigMessage::Gear(gear_message) => match gear_message {
                GearMessage::HelmetSelected(idx, name) => {
                    if let Some(selection) =
                        self.gear.helmet_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config.items.helmets.push(name);
                    self.save_config();
                }
                GearMessage::AddHelmet => {
                    self.gear.helmet_selections.push(None);
                }
                GearMessage::ChestplateSelected(idx, name) => {
                    if let Some(selection) =
                        self.gear.chestplate_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config.items.chest_plates.push(name);
                    self.save_config();
                }
                GearMessage::AddChestplate => {
                    self.gear.chestplate_selections.push(None);
                }
                GearMessage::LeggingsSelected(idx, name) => {
                    if let Some(selection) =
                        self.gear.leggings_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config.items.leggings.push(name);
                    self.save_config();
                }
                GearMessage::AddLeggings => {
                    self.gear.leggings_selections.push(None);
                }
                GearMessage::BootsSelected(idx, name) => {
                    if let Some(selection) =
                        self.gear.boots_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config.items.boots.push(name);
                    self.save_config();
                }
                GearMessage::AddBoots => {
                    self.gear.boots_selections.push(None);
                }
                GearMessage::RingsSelected(idx, name) => {
                    if let Some(selection) =
                        self.gear.rings_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config.items.rings.push(name);
                    self.save_config();
                }
                GearMessage::AddRings => {
                    self.gear.rings_selections.push(None);
                }
                GearMessage::BraceletsSelected(idx, name) => {
                    if let Some(selection) =
                        self.gear.bracelets_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config.items.bracelets.push(name);
                    self.save_config();
                }
                GearMessage::AddBracelets => {
                    self.gear.bracelets_selections.push(None);
                }
                GearMessage::NecklacesSelected(idx, name) => {
                    if let Some(selection) =
                        self.gear.necklaces_selections.get_mut(idx)
                    {
                        *selection = Some(name.clone());
                    }
                    self.config.items.necklaces.push(name);
                    self.save_config();
                }
                GearMessage::AddNecklaces => {
                    self.gear.necklaces_selections.push(None);
                }
                GearMessage::WeaponSelected(name) => {
                    self.gear.selected_weapon = Some(name);
                    self.save_config();
                }
                GearMessage::RemoveHelmet(idx) => {
                    self.gear.helmet_selections.remove(idx);
                    let helmet = self.config.items.helmets.get(idx).cloned();
                    if let Some(helmet) = helmet {
                        self.config
                            .items
                            .helmets
                            .retain(|x| x != &helmet);
                    }
                    self.save_config();
                }
                GearMessage::RemoveChestplate(idx) => {
                    self.gear.chestplate_selections.remove(idx);
                    let chestplate = self
                        .config
                        .items
                        .chest_plates
                        .get(idx)
                        .cloned();
                    if let Some(chestplate) = chestplate {
                        self.config
                            .items
                            .chest_plates
                            .retain(|x| x != &chestplate);
                    }
                    self.save_config();
                }
                GearMessage::RemoveLeggings(idx) => {
                    self.gear.leggings_selections.remove(idx);
                    let legging = self.config.items.leggings.get(idx).cloned();
                    if let Some(legging) = legging {
                        self.config
                            .items
                            .leggings
                            .retain(|x| x != &legging);
                    }
                    self.save_config();
                }
                GearMessage::RemoveBoots(idx) => {
                    self.gear.boots_selections.remove(idx);
                    let boot = self.config.items.boots.get(idx).cloned();
                    if let Some(boot) = boot {
                        self.config
                            .items
                            .boots
                            .retain(|x| x != &boot);
                    }
                    self.save_config();
                }
                GearMessage::RemoveRings(idx) => {
                    self.gear.rings_selections.remove(idx);
                    let ring = self.config.items.rings.get(idx).cloned();
                    if let Some(ring) = ring {
                        self.config
                            .items
                            .rings
                            .retain(|x| x != &ring);
                    }
                    self.save_config();
                }
                GearMessage::RemoveBracelets(idx) => {
                    self.gear.bracelets_selections.remove(idx);
                    let bracelet = self
                        .config
                        .items
                        .bracelets
                        .get(idx)
                        .cloned();
                    if let Some(bracelet) = bracelet {
                        self.config
                            .items
                            .bracelets
                            .retain(|x| x != &bracelet);
                    }
                    self.save_config();
                }
                GearMessage::RemoveNecklaces(idx) => {
                    self.gear.necklaces_selections.remove(idx);
                    let necklace = self
                        .config
                        .items
                        .necklaces
                        .get(idx)
                        .cloned();
                    if let Some(necklace) = necklace {
                        self.config
                            .items
                            .necklaces
                            .retain(|x| x != &necklace);
                    }
                    self.save_config();
                }
            },
            ConfigMessage::Player(player_message) => match player_message {
                PlayerMessage::LevelChanged(content) => {
                    if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config.player.lvl = content.parse().unwrap_or(1);
                        self.save_config();
                    }
                }
                PlayerMessage::AvailablePointChanged(content) => {
                    if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config.player.available_point =
                            content.parse().unwrap_or(200);
                        self.save_config();
                    }
                }
                PlayerMessage::BaseHpChanged(content) => {
                    if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                        self.config.player.base_hp =
                            content.parse().unwrap_or(500);
                        self.save_config();
                    }
                }
            },
            ConfigMessage::ThresholdFirst(threshold_first_message) => {
                match threshold_first_message {
                    ThresholdFirstMessage::HpChanged(content) => {
                        if content.is_empty() {
                            self.config
                                .threshold_first
                                .as_mut()
                                .unwrap()
                                .min_hp = None;
                        } else if content.chars().all(|c| c.is_ascii_digit() || c == '-') {
                            self.config
                                .threshold_first
                                .as_mut()
                                .unwrap()
                                .min_hp = Some(content.parse().unwrap_or(0));
                        }
                        self.save_config();
                    }
                }
            }
            ConfigMessage::ThresholdSecond(threshold_second_message) => {
                if let Some(threshold) = self.config.threshold_second.as_mut() {
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
                    self.save_config();
                }
            }
            ConfigMessage::ThresholdThird(threshold_third_message) => {
                if let Some(threshold) = self.config.threshold_third.as_mut() {
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
                    self.save_config();
                }
            }
            ConfigMessage::ThresholdFourth(threshold_fourth_message) => {
                if let Some(threshold) = self.config.threshold_fourth.as_mut() {
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
                    self.save_config();
                }
            }
            ConfigMessage::ThresholdFifth(threshold_fifth_message) => {
                if let Some(threshold) = self.config.threshold_fifth.as_mut() {
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
                    self.save_config();
                }
            }
            ConfigMessage::Hppeng(hppeng_message) => {
                match hppeng_message {
                    HppengMessage::UrlPrefixChanged(content) => {
                        self.config.hppeng.url_prefix = content;
                    }
                    HppengMessage::UrlSuffixChanged(content) => {
                        self.config.hppeng.url_suffix = content;
                    }
                    HppengMessage::DbPathChanged(content) => {
                        self.config.hppeng.db_path = content;
                    }
                    HppengMessage::MigrationsPathChanged(content) => {
                        self.config.hppeng.migrations_path = content;
                    }
                    HppengMessage::ItemsFileChanged(content) => {
                        self.config.hppeng.items_file = content;
                    }
                    HppengMessage::LogBuildsChanged(value) => {
                        self.config.hppeng.log_builds = value;
                    }
                    HppengMessage::LogDbErrorsChanged(value) => {
                        self.config.hppeng.log_db_errors = value;
                    }
                    HppengMessage::DbRetryCountChanged(content) => {
                        self.config.hppeng.db_retry_count =
                            content.parse().unwrap_or(3);
                    }
                }
                self.save_config();
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Container<'_, Message> {
        let content = column![
                text("Edit Configuration File").size(30),
                text("The configuration will be saved automatically when you edit.").size(20),
                text("If the items say \"No items found\", check that items.json is present in the config folder, and close and re-open the application.").size(20),
                // Error if no items.json file is found
                if let Some(error_message) = &self.error_message {
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
                                &self.config.player.lvl.to_string()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Player(PlayerMessage::LevelChanged(input))))
                            .size(16)
                            .padding(5)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Available Points:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter points...", 
                                &self.config.player.available_point.to_string()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Player(PlayerMessage::AvailablePointChanged(input))))
                            .size(16)
                            .padding(5)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Base HP:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter base HP...",
                                &self.config.player.base_hp.to_string()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Player(PlayerMessage::BaseHpChanged(input))))
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
                                &self.config.hppeng.url_prefix
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::UrlPrefixChanged(input))))
                            .size(16)
                            .padding(5)
                            .width(Length::Fill),
                        ],
                        row![
                            text("URL Suffix:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter URL suffix...",
                                &self.config.hppeng.url_suffix
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::UrlSuffixChanged(input))))
                            .size(16)
                            .padding(5)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Database Path:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter database path...",
                                &self.config.hppeng.db_path
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::DbPathChanged(input))))
                            .size(16)
                            .padding(5)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Migrations Path:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter migrations path...",
                                &self.config.hppeng.migrations_path
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::MigrationsPathChanged(input))))
                            .size(16)
                            .padding(5)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Items File:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter items file path...",
                                &self.config.hppeng.items_file
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::ItemsFileChanged(input))))
                            .size(16)
                            .padding(5)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Log Builds:").width(Length::Fixed(150.0)),
                            checkbox(
                                "",
                                self.config.hppeng.log_builds,
                            )
                            .on_toggle(|value| Message::Config(ConfigMessage::Hppeng(HppengMessage::LogBuildsChanged(value))))
                            .size(16)
                            .spacing(5),
                        ],
                        row![
                            text("Log DB Errors:").width(Length::Fixed(150.0)),
                            checkbox(
                                "",
                                self.config.hppeng.log_db_errors,
                            )
                            .on_toggle(|value| Message::Config(ConfigMessage::Hppeng(HppengMessage::LogDbErrorsChanged(value))))
                            .size(16)
                            .spacing(5),
                        ],
                        row![
                            text("DB Retry Count:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter retry count...",
                                &self.config.hppeng.db_retry_count.to_string()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::Hppeng(HppengMessage::DbRetryCountChanged(input))))
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
                        &self.config.threshold_first.as_ref().unwrap().min_hp
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFirst(ThresholdFirstMessage::HpChanged(input))))
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
                        &self.config.threshold_second.as_ref().unwrap().min_hpr_raw
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::HprRawChanged(input))))
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min HPR Percentage:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min HPR percentage...",
                        &self.config.threshold_second.as_ref().unwrap().min_hpr_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::HprPctChanged(input))))
                    .size(16)
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min MR:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min MR...",
                        &self.config.threshold_second.as_ref().unwrap().min_mr
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::MrChanged(input))))
                    .size(16)
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min LS:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min LS...",
                        &self.config.threshold_second.as_ref().unwrap().min_ls
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::LsChanged(input))))
                    .size(16)
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min MS:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min MS...",
                        &self.config.threshold_second.as_ref().unwrap().min_ms
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::MsChanged(input))))
                    .size(16)
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min SPD:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min SPD...",
                        &self.config.threshold_second.as_ref().unwrap().min_spd
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::SpdChanged(input))))
                    .size(16)
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min SD Raw:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min SD raw...",
                        &self.config.threshold_second.as_ref().unwrap().min_sd_raw
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::SdRawChanged(input))))
                    .size(16)
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min SD Percentage:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min SD percentage...",
                        &self.config.threshold_second.as_ref().unwrap().min_sd_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::SdPctChanged(input))))
                    .size(16)
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min HPR:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min HPR...",
                        &self.config.threshold_second.as_ref().unwrap().min_hpr
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::HprChanged(input))))
                    .size(16)
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min EXP Bonus:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min EXP bonus...",
                        &self.config.threshold_second.as_ref().unwrap().min_exp_bonus
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdSecond(ThresholdSecondMessage::ExpBonusChanged(input))))
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
                        &self.config.threshold_third.as_ref().unwrap().min_earth_defense
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdThird(ThresholdThirdMessage::EarthDefChanged(input))))
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Thunder Defense:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min thunder defense...",
                        &self.config.threshold_third.as_ref().unwrap().min_thunder_defense
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdThird(ThresholdThirdMessage::ThunderDefChanged(input))))
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Water Defense:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min water defense...",
                        &self.config.threshold_third.as_ref().unwrap().min_water_defense
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdThird(ThresholdThirdMessage::WaterDefChanged(input))))
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Fire Defense:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min fire defense...",
                        &self.config.threshold_third.as_ref().unwrap().min_fire_defense
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdThird(ThresholdThirdMessage::FireDefChanged(input))))
                    .padding(5)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Air Defense:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min air defense...",
                        &self.config.threshold_third.as_ref().unwrap().min_air_defense
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdThird(ThresholdThirdMessage::AirDefChanged(input))))
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
                        &self.config.threshold_fourth.as_ref().unwrap().min_neutral_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::NeutralDamPctChanged(input))))
                    .padding(10)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Earth Damage %:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min earth damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap().min_earth_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::EarthDamPctChanged(input))))
                    .padding(10)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Thunder Damage %:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min thunder damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap().min_thunder_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::ThunderDamPctChanged(input))))
                    .padding(10)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Water Damage %:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min water damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap().min_water_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::WaterDamPctChanged(input))))
                    .padding(10)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Fire Damage %:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min fire damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap().min_fire_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::FireDamPctChanged(input))))
                    .padding(10)
                    .width(Length::Fill),
                ],
                row![
                    text("Min Air Damage %:").width(Length::Fixed(150.0)),
                    text_input(
                        "Enter min air damage %...",
                        &self.config.threshold_fourth.as_ref().unwrap().min_air_dam_pct
                            .map(|v| v.to_string())
                            .unwrap_or_default()
                    )
                    .on_input(|input| Message::Config(ConfigMessage::ThresholdFourth(ThresholdFourthMessage::AirDamPctChanged(input))))
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
                                &self.config.threshold_fifth.as_ref().unwrap().min_earth_point
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::EarthPointChanged(input))))
                            .padding(10)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Min Thunder Point:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter min thunder point...",
                                &self.config.threshold_fifth.as_ref().unwrap().min_thunder_point
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::ThunderPointChanged(input))))
                            .padding(10)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Min Water Point:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter min water point...",
                                &self.config.threshold_fifth.as_ref().unwrap().min_water_point
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::WaterPointChanged(input))))
                            .padding(10)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Min Fire Point:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter min fire point...",
                                &self.config.threshold_fifth.as_ref().unwrap().min_fire_point
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::FirePointChanged(input))))
                            .padding(10)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Min Air Point:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter min air point...",
                                &self.config.threshold_fifth.as_ref().unwrap().min_air_point
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::AirPointChanged(input))))
                            .padding(10)
                            .width(Length::Fill),
                        ],
                        row![
                            text("Min EHP:").width(Length::Fixed(150.0)),
                            text_input(
                                "Enter min EHP...",
                                &self.config.threshold_fifth.as_ref().unwrap().min_ehp
                                    .map(|v| v.to_string())
                                    .unwrap_or_default()
                            )
                            .on_input(|input| Message::Config(ConfigMessage::ThresholdFifth(ThresholdFifthMessage::EhpChanged(input))))
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
                    self.gear
                        .helmet_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(5), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.helmets,
                                        "Select helmet...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::HelmetSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveHelmet(idx))))
                                        .padding(5),
                                ]
                                .spacing(10)
                            )
                        }),
                    button("Add Helmet").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddHelmet))),
                ]
                .spacing(10),
                // Gear Selection - Chestplates
                column![
                    text("Chestplates:"),
                    self.gear
                        .chestplate_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(5), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.chestplates,
                                        "Select chestplate...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::ChestplateSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveChestplate(idx))))
                                        .padding(5),
                                ]
                                .spacing(10)
                            )
                        }),
                    button("Add Chestplate").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddChestplate))),
                ]
                .spacing(10),
                // Gear Selection - Leggings
                column![
                    text("Leggings:"),
                    self.gear
                        .leggings_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(5), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.leggings,
                                        "Select leggings...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::LeggingsSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveLeggings(idx))))
                                        .padding(5),
                                ]
                                .spacing(10)
                            )
                        }),
                    button("Add Leggings").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddLeggings))),
                ]
                .spacing(10),
                // Gear Selection - Boots
                column![
                    text("Boots:"),
                    self.gear
                        .boots_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(5), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.boots,
                                        "Select boots...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::BootsSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveBoots(idx))))
                                        .padding(5),
                                ]
                                .spacing(10)
                            )
                        }),
                    button("Add Boots").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddBoots))),
                ]
                .spacing(10),
                // Gear Selection - Rings
                column![
                    text("Rings:"),
                    self.gear
                        .rings_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(5), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.rings,
                                        "Select ring...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::RingsSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveRings(idx))))
                                        .padding(5),
                                ]
                                .spacing(10)
                            )
                        }),
                    button("Add Ring").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddRings))),
                ]
                .spacing(10),
                // Gear Selection - Bracelets
                column![
                    text("Bracelets:"),
                    self.gear
                        .bracelets_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(5), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.bracelets,
                                        "Select bracelet...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::BraceletsSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveBracelets(idx))))
                                        .padding(5),
                                ]
                                .spacing(10)
                            )
                        }),
                    button("Add Bracelet").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddBracelets))),
                ]
                .spacing(10),
                // Gear Selection - Necklaces
                column![
                    text("Necklaces:"),
                    self.gear
                        .necklaces_selections
                        .iter()
                        .enumerate()
                        .fold(column![].spacing(5), |col, (idx, selection)| {
                            col.push(
                                row![
                                    combo_box(
                                        &self.gear.necklaces,
                                        "Select necklace...",
                                        selection.as_ref(),
                                        move |name| Message::Config(ConfigMessage::Gear(GearMessage::NecklacesSelected(idx, name))),
                                    ),
                                    button("X")
                                        .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveNecklaces(idx))))
                                        .padding(5),
                                ]
                                .spacing(10)
                            )
                        }),
                    button("Add Necklace").on_press(Message::Config(ConfigMessage::Gear(GearMessage::AddNecklaces))),
                ]
                .spacing(10),
                // Gear Selection - Weapon
                column![
                    text("Weapon:"),
                    combo_box(
                        &self.gear.weapons,
                        "Select weapon...",
                        self.gear.selected_weapon.as_ref(),
                        |name| Message::Config(ConfigMessage::Gear(GearMessage::WeaponSelected(name))),
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

    pub fn save_config(&mut self) {
        self.config
            .save_config("config/config.toml")
            .unwrap_or_default();
    }
}

#[derive(Default)]
pub struct GearSelections {
    pub helmets: combo_box::State<String>,
    pub helmet_selections: Vec<Option<String>>,
    pub chestplates: combo_box::State<String>,
    pub chestplate_selections: Vec<Option<String>>,
    pub leggings: combo_box::State<String>,
    pub leggings_selections: Vec<Option<String>>,
    pub boots: combo_box::State<String>,
    pub boots_selections: Vec<Option<String>>,
    pub rings: combo_box::State<String>,
    pub rings_selections: Vec<Option<String>>,
    pub bracelets: combo_box::State<String>,
    pub bracelets_selections: Vec<Option<String>>,
    pub necklaces: combo_box::State<String>,
    pub necklaces_selections: Vec<Option<String>>,
    pub weapons: combo_box::State<String>,
    pub selected_weapon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GearList {
    pub items: Vec<Gear>,
}

impl GearList {
    pub fn from_json(path: &str) -> Result<Self, String> {
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

    pub fn get_gear_by_type(&self, gear_type: GearType) -> Vec<String> {
        self.items
            .iter()
            .filter(|gear| gear.gear_type == gear_type)
            .map(|gear| gear.name.clone())
            .collect()
    }

    pub fn helmets(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Helmet)
    }

    pub fn chestplates(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Chestplate)
    }

    pub fn leggings(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Leggings)
    }

    pub fn boots(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Boots)
    }

    pub fn rings(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Ring)
    }

    pub fn bracelets(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Bracelet)
    }

    pub fn necklaces(&self) -> Vec<String> {
        self.get_gear_by_type(GearType::Necklace)
    }

    pub fn weapons(&self) -> Vec<String> {
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
pub struct Gear {
    id: i64,
    name: String,
    tier: String,
    #[serde(rename = "type")]
    gear_type: GearType,
}

impl Gear {
    pub fn default_for_type(gear_type: GearType, name: &str, id: i64) -> Self {
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
pub enum GearType {
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