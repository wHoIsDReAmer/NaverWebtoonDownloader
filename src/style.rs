use iced::{
    button, container, rule, text_input
};

pub struct Button;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme;

impl<'a> From<Theme> for Box<dyn button::StyleSheet + 'a> {
    fn from(_: Theme) -> Self {
        Button.into()
    }
}

use iced::{Color, Vector};
impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Color::from_rgb(0.41, 0.8, 0.67).into(),
            border_radius: 12.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..Default::default()
        }
    }
}