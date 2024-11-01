use super::Theme;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(theme: &Theme, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let theme_str = match theme {
        Theme::Light => "light",
        Theme::Dark => "dark", 
        Theme::Dracula => "dracula",
        Theme::Nord => "nord",
        Theme::SolarizedLight => "solarized_light",
        Theme::SolarizedDark => "solarized_dark",
        Theme::GruvboxLight => "gruvbox_light",
        Theme::GruvboxDark => "gruvbox_dark",
        Theme::CatppuccinLatte => "catppuccin_latte",
        Theme::CatppuccinFrappe => "catppuccin_frappe",
        Theme::CatppuccinMacchiato => "catppuccin_macchiato",
        Theme::CatppuccinMocha => "catppuccin_mocha",
        Theme::TokyoNight => "tokyo_night",
        Theme::TokyoNightStorm => "tokyo_night_storm",
        Theme::TokyoNightLight => "tokyo_night_light",
        Theme::KanagawaWave => "kanagawa_wave",
        Theme::KanagawaDragon => "kanagawa_dragon",
        Theme::KanagawaLotus => "kanagawa_lotus",
        Theme::Moonfly => "moonfly",
        Theme::Nightfly => "nightfly",
        Theme::Oxocarbon => "oxocarbon",
        Theme::Ferra => "ferra",
        Theme::Custom(_) => "custom",  // Custom themes serialize to "custom" string
    };
    serializer.serialize_str(theme_str)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Theme, D::Error>
where
    D: Deserializer<'de>,
{
    let theme_str = String::deserialize(deserializer)?;
    match theme_str.to_lowercase().as_str() {
        "light" => Ok(Theme::Light),
        "dark" => Ok(Theme::Dark),
        "dracula" => Ok(Theme::Dracula),
        "nord" => Ok(Theme::Nord),
        "solarized_light" => Ok(Theme::SolarizedLight),
        "solarized_dark" => Ok(Theme::SolarizedDark),
        "gruvbox_light" => Ok(Theme::GruvboxLight),
        "gruvbox_dark" => Ok(Theme::GruvboxDark),
        "catppuccin_latte" => Ok(Theme::CatppuccinLatte),
        "catppuccin_frappe" => Ok(Theme::CatppuccinFrappe),
        "catppuccin_macchiato" => Ok(Theme::CatppuccinMacchiato),
        "catppuccin_mocha" => Ok(Theme::CatppuccinMocha),
        "tokyo_night" => Ok(Theme::TokyoNight),
        "tokyo_night_storm" => Ok(Theme::TokyoNightStorm),
        "tokyo_night_light" => Ok(Theme::TokyoNightLight),
        "kanagawa_wave" => Ok(Theme::KanagawaWave),
        "kanagawa_dragon" => Ok(Theme::KanagawaDragon),
        "kanagawa_lotus" => Ok(Theme::KanagawaLotus),
        "moonfly" => Ok(Theme::Moonfly),
        "nightfly" => Ok(Theme::Nightfly),
        "oxocarbon" => Ok(Theme::Oxocarbon),
        "ferra" => Ok(Theme::Ferra),
        _ => Ok(Theme::Dark),
    }
}
