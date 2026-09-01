use iced::widget::{container, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    // Launch the application window
    MyWindow::run(Settings::default())
}

// 1. Define your application state
struct MyWindow;

// 2. Implement the Sandbox trait
impl Sandbox for MyWindow {
    type Message = (); // No interactive events needed for a static window

    // Initialize state
    fn new() -> Self {
        Self
    }

    // Window title
    fn title(&self) -> String {
        String::from("Basic Iced Window")
    }

    // State update logic
    fn update(&mut self, _message: Self::Message) {
        // No-op for standard static window
    }

    // Build the user interface
    fn view(&self) -> Element<Self::Message> {
        container(text("Hello, World!").size(32))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}