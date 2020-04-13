use::serde::*;
use iced::{Column, Align, Length, Color, Scrollable, Text, Space};

const PROJECTS : &'static [u8] = include_bytes!("misc/projects.json");
#[derive(Serialize, Deserialize)]
struct Project {
    name : String,
    description: String,
    url: String
}
pub fn build_columns<'a, T : 'static>() -> Column<'a, T> {
    let projects = serde_json::from_slice::<Vec<Project>>(PROJECTS).unwrap();
    let mut cols = Column::new()
        .push(Space::new(Length::Fill, Length::Units(10)));
    for i in projects {
        cols = cols.push(Column::new()
            .push(Text::new(i.name).color(Color::from_rgb8(255, 120, 180)))
            .push(Text::new(i.description))
            .push(Text::new(i.url))
            .spacing(5)
            .padding(20));
    }
    cols
        .spacing(10)
        .align_items(Align::Start)
        .width(Length::Fill)
        .height(Length::Fill)
}