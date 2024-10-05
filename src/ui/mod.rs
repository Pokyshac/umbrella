use std::ops::Deref;

use fltk::{
    self, app::{self, App, Scheme}, button::Button, enums::{Color, FrameType}, group::{Column, Flex, FlexType, Row}, prelude::{FltkError, GroupExt, WidgetBase, WidgetExt, WindowExt}, text::{TextBuffer, TextDisplay}, window::Window
};

pub fn get_ui() -> Flex {
    // let mut result = Flex::default().with_type(FlexType::Column);
    let mut result = Flex::default_fill().column();
    result.set_spacing(30);

    result.add(
        &Button::default().with_label("asda").size_of_parent()
    );
    result.add(
        &TextDisplay::default().with_label("govno").size_of_parent()
    );

    result
}
