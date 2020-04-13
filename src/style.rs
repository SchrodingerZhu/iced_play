use iced::{Background, button, Color, Vector, container};
use iced::widget::button::Style;

pub enum Button {
    Primary,
    Secondary,
}

pub struct ColumnStyle;
pub struct ButtonStyle;

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(1.0, 1.0),
            border_radius: 12,
            text_color: Color::WHITE,
            background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
            ..Default::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(1.0, 2.0),
            border_radius: 12,
            text_color: Color::WHITE,
            background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
            ..Default::default()
        }
    }
}



