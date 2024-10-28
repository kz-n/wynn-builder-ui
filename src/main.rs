use iced::widget::{container, text};
use iced::{Element, Length, Sandbox, Settings};

struct HelloWorld;

impl Sandbox for HelloWorld {
    type Message = ();

    fn new() -> Self {
        HelloWorld
    }

    fn title(&self) -> String {
        String::from("Hello, World!")
    }

    fn update(&mut self, _message: Self::Message) {
        // No updates needed for Hello World
    }

    fn view(&self) -> Element<Self::Message> {
        container(text("Hello, World!"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    HelloWorld::run(Settings::default())
}