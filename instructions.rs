// At the top of the file, add these style constants
const TEXT_SIZE: u16 = 16;
const HEADER_SIZE: u16 = 30;
const SUBHEADER_SIZE: u16 = 20;
const SPACING: u16 = 10;
const INPUT_PADDING: u16 = 5;
const SECTION_PADDING: u16 = 20;
const BUTTON_PADDING: u16 = 5;

// In the view() function, update the styling:

text("Edit Configuration File")
    .size(HEADER_SIZE)
    // ...

text("The configuration will be saved automatically when you edit.")
    .size(SUBHEADER_SIZE)
    // ...

// For all text inputs, standardize the styling:
text_input(
    "Enter level (1-106)...",
    &self.config.player.lvl.to_string()
)
.on_input(|input| Message::Config(ConfigMessage::Player(PlayerMessage::LevelChanged(input))))
.size(TEXT_SIZE)
.padding(INPUT_PADDING)
.width(Length::Fill)

// For all buttons, standardize the styling:
button("X")
    .on_press(Message::Config(ConfigMessage::Gear(GearMessage::RemoveHelmet(idx))))
    .padding(BUTTON_PADDING)
    .style(theme::Button::Secondary) // Add a consistent button style

// For all combo boxes, standardize the styling:
combo_box(
    &self.gear.helmets,
    "Select helmet...",
    selection.as_ref(),
    move |name| Message::Config(ConfigMessage::Gear(GearMessage::HelmetSelected(idx, name))),
)
.padding(INPUT_PADDING)
.width(Length::Fill)

// For all containers, standardize the padding:
container(content)
    .padding(SECTION_PADDING)
    .width(Length::Fill)

// For all columns and rows, standardize the spacing:
column![/* ... */]
    .spacing(SPACING)
    .align_x(Horizontal::Left)

row![/* ... */]
    .spacing(SPACING)
    .align_items(Alignment::Center)

// For labels (text elements before inputs), standardize the width:
text("Player Level:").width(Length::Fixed(150.0))