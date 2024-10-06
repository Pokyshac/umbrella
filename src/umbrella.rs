use iced::{widget::{Column, Text}, Element, Task};

#[derive(Debug, Default)]
pub struct Umbrella {
    
}

#[derive(Debug)]
pub enum Message {
    ASDA,
}

impl Umbrella {
    pub fn title(&self) -> String {
        String::from("Umbrella")
    }

    pub fn update(&mut self, message: Message) {
    }

    pub fn view(&self) -> Element<Message> {
        Column::new().push(
            Text::new("govno")
        ).push(
            Text::new("shit")
        ).into()
    }
}
