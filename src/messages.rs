use iced::Theme;
use iced_widget::text_editor::Action;
use crate::Tab;

#[derive(Debug, Clone)]
pub enum SearchMessage {
    InputChanged(String),
    InputSubmitted,
    ItemEditorAction(Action),
}

#[derive(Debug, Clone)]
pub enum GearMessage {
    HelmetSelected(usize, String),
    AddHelmet,
    ChestplateSelected(usize, String),
    AddChestplate,
    LeggingsSelected(usize, String),
    AddLeggings,
    BootsSelected(usize, String),
    AddBoots,
    RingsSelected(usize, String),
    AddRings,
    BraceletsSelected(usize, String),
    AddBracelets,
    NecklacesSelected(usize, String),
    AddNecklaces,
    WeaponSelected(String),
    RemoveHelmet(usize),
    RemoveChestplate(usize),
    RemoveLeggings(usize),
    RemoveBoots(usize),
    RemoveRings(usize),
    RemoveBracelets(usize),
    RemoveNecklaces(usize),
}

#[derive(Debug, Clone)]
pub enum PlayerMessage {
    LevelChanged(String),
    AvailablePointChanged(String),
    BaseHpChanged(String),
}

#[derive(Debug, Clone)]
pub enum ThresholdFirstMessage {
    HpChanged(String),
}

#[derive(Debug, Clone)]
pub enum ThresholdSecondMessage {
    HprRawChanged(String),
    HprPctChanged(String),
    MrChanged(String),
    LsChanged(String),
    MsChanged(String),
    SpdChanged(String),
    SdRawChanged(String),
    SdPctChanged(String),
    HprChanged(String),
    ExpBonusChanged(String),
}

#[derive(Debug, Clone)]
pub enum ThresholdThirdMessage {
    EarthDefChanged(String),
    ThunderDefChanged(String),
    WaterDefChanged(String),
    FireDefChanged(String),
    AirDefChanged(String),
}

#[derive(Debug, Clone)]
pub enum ThresholdFourthMessage {
    NeutralDamPctChanged(String),
    EarthDamPctChanged(String),
    ThunderDamPctChanged(String),
    WaterDamPctChanged(String),
    FireDamPctChanged(String),
    AirDamPctChanged(String),
}

#[derive(Debug, Clone)]
pub enum ThresholdFifthMessage {
    EarthPointChanged(String),
    ThunderPointChanged(String),
    WaterPointChanged(String),
    FirePointChanged(String),
    AirPointChanged(String),
    EhpChanged(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
    ThemeChanged(Theme),
    Search(SearchMessage),
    Gear(GearMessage),
    Player(PlayerMessage),
    ThresholdFirst(ThresholdFirstMessage),
    ThresholdSecond(ThresholdSecondMessage),
    ThresholdThird(ThresholdThirdMessage),
    ThresholdFourth(ThresholdFourthMessage),
    ThresholdFifth(ThresholdFifthMessage),
}
