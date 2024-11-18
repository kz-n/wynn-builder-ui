mod gear;
mod config_view;
mod config_update;
pub mod style;

pub use gear::{Gear, GearList, GearSelections, GearType};
use crate::build_config::Config;

#[derive(Default)]
pub struct ConfigFile {
    pub error_message: Option<String>,
    pub gear: GearSelections,
    pub config: Config,
}

impl ConfigFile {
    pub fn save_config(&mut self) {
        self.config
            .save_config("config/config.toml")
            .unwrap_or_default();
    }
} 