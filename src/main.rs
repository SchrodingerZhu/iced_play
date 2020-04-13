use iced::{Space, Align, Button, button, Color, Column, Container, Element, HorizontalAlignment, Image, Length, Sandbox, Settings, Text, VerticalAlignment, Row, Scrollable};
use iced::widget::image::Handle;
use iced::widget::radio::Style;
use wee_alloc::WeeAlloc;

mod style;
mod projects;
#[global_allocator]
static GLOBAL: WeeAlloc = WeeAlloc::INIT;
fn next_color(counter: usize) -> Color {
    const COLORS : [Color; 7] = [
        Color{r: 1.0, g: 0.0, b: 0.0, a: 1.0},
        Color{r: 1.0, g: 0.5, b: 0.0, a: 1.0},
        Color{r: 0.8, g: 0.8, b: 0.2, a: 1.0},
        Color{r: 0.0, g: 1.0, b: 0.0, a: 1.0},
        Color{r: 0.0, g: 0.0, b: 1.0, a: 1.0},
        Color{r: 0.18, g: 0.17, b: 0.37, a: 1.0},
        Color{r: 0.55, g: 0.0, b: 1.0, a: 1.0},
    ];
    unsafe {
        COLORS[counter % 7].clone()
    }
}
enum Site {
    HomePage {
        // The counter value
        logo: button::State,
        name_color: Color,
        project: button::State,
        scroll: iced::scrollable::State
    },
    ProjectList {
        back: button::State,
        scroll: iced::scrollable::State
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    LogoPressed,
    ProjectPressed,
    ReturnPressed,
}


impl Sandbox for Site {
    fn view(&mut self) -> Element<Message> {

        // We use a column: a simple vertical layout
        match self {
            Site::HomePage { logo, name_color, project , scroll} => {
                Scrollable::new(scroll)
                    .push(Space::new(Length::Fill, Length::Units(2)))
                    .push(Button::new(logo,
                                      Image::new(Handle::from_path("misc/logo.jpg")),
                    )
                        .on_press(Message::LogoPressed)
                    )
                    .push(Text::new(String::from("Schrodinger Zhu"))
                        .color(name_color.clone())
                        .size(50)
                    )
                    .push(Column::new()
                        .push(Text::new(String::from("Email: i@zhuyi.fan"))
                            .color(next_color(0)))
                        .push(Text::new(String::from("Github: @Schrodinger Zhu"))
                            .color(next_color(1)))
                        .push(Text::new(String::from("Twitter: @ZhuSchrodinger"))
                            .color(next_color(2)))
                        .push(Text::new(String::from("PGP: 6552 350D 8D7E E5FB"))
                            .color(next_color(3)))
                        .align_items(Align::Center)
                        .width(Length::Fill))
                    .push(Button::new(project, Text::new(String::from("Projects")))
                        .on_press(Message::ProjectPressed)
                        .style(style::ButtonStyle))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .spacing(50)
                    .align_items(Align::Center)
                    .into()
            }
            Site::ProjectList { back, scroll } => {
                Scrollable::new(scroll)
                    .push(Button::new(back, Text::new(String::from("Back")))
                        .on_press(Message::ReturnPressed)
                        .style(style::ButtonStyle))
                    .push(projects::build_columns())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }

    fn update(&mut self, message: Message) {
        match self {
            Site::HomePage { logo, name_color, project, scroll } => {
                match message {
                    Message::LogoPressed => {
                        name_color.r = rand::random();
                        name_color.g = rand::random();
                        name_color.b = rand::random();
                    }
                    Message::ProjectPressed => {
                        *self = Site::ProjectList {
                            back: Default::default(),
                            scroll: Default::default()
                        }
                    }
                    _ => {}
                }
            }
            Site::ProjectList { .. } => {
                match message {
                    Message::ReturnPressed => {
                        *self = Self::new()
                    }
                    _ => {}
                }
            }
        }
    }

    type Message = crate::Message;

    fn new() -> Self {
        Site::HomePage {
            logo: button::State::new(),
            name_color: Color::BLACK,
            project: Default::default(),
            scroll: Default::default()
        }
    }

    fn title(&self) -> String {
        match self {
            Site::HomePage {..} => String::from("Homepage | Schrodinger Zhu"),
            Site::ProjectList {..} => String::from("Projects | Schrodinger Zhu"),
        }
    }
}

fn main() {
    Site::run(Settings::default());
}
