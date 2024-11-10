use std::path::Path;

use iced::{
    alignment::{Horizontal, Vertical},
    Length,
};
use iced_widget::{checkbox, column, container, row, scrollable, text, Container};

use crate::Message;

#[derive(Default)]
pub struct Intro {}

impl Intro {
    pub fn view(&self) -> Container<'_, Message> {
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
                ],
                text("Instructions for first time setup").size(20),
                text("1. Make sure you've extracted both the release Wynn Builder UI and the WynnBuilderTools release into the same folder.").size(16),
                text("2. Go to the Search tab, and run any command with --sort or -s to generate the items.json file.").size(16),
                text("3. Once the items.json file is generated, go to the Config File tab, and check that you get no errors.\nIf you do, check that items.json is present in the config folder, and close and re-open the application.").size(16),
                text("4. The builder tab is unfinished, and will be updated in the future.").size(16),
                text("For now, you can use the builder by running it in a terminal with no arguments.").size(16),
                text("I'm hoping that for now the Config tab will suffice in making the tool usable.").size(16),
            ]
            .spacing(20)
            .align_x(Horizontal::Center);

        container(scrollable(container(column).padding(10)))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
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
    config_names
        .iter()
        .any(|name| Path::new("config").join(name).exists())
}

fn is_items_json_found() -> bool {
    Path::new("config/items.json").exists()
}
