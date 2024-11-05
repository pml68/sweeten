use iced::widget::{container, pick_list, slider, text, toggler};
use iced::Alignment::Center;
use iced::Length::{Fill, Fixed};
use iced::{Element, Theme};

use sweeten::layout::flex::{flex, JustifyContent};
use sweeten::layout::FlexAlignment;
use sweeten::widget::{Column, Row};
use sweeten::{column, row};

pub fn main() -> iced::Result {
    iced::application(
        "sweetened iced - interactive flexbox",
        App::update,
        App::view,
    )
    .theme(|_| Theme::TokyoNightLight)
    .window(iced::window::Settings {
        size: (900.0, 600.0).into(),
        ..Default::default()
    })
    .run()
}

struct App {
    direction: Direction,
    flex_grow: bool,
    flex_shrink: f32,
    container_width: u16,
    container_height: u16,
}

impl Default for App {
    fn default() -> Self {
        App {
            direction: Direction::Row,
            flex_grow: true,
            flex_shrink: 4.0,
            container_width: 700,
            container_height: 300,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    DirectionChanged(Direction),
    FlexGrowToggled(bool),
    FlexShrinkChanged(f32),
    ContainerWidthChanged(u16),
    ContainerHeightChanged(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Direction {
    #[default]
    Row,
    Column,
}

impl Direction {
    const ALL: [Direction; 2] = [Direction::Row, Direction::Column];
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Row => write!(f, "Row"),
            Direction::Column => write!(f, "Column"),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::DirectionChanged(direction) => {
                self.direction = direction;
            }
            Message::FlexGrowToggled(enabled) => {
                self.flex_grow = enabled;
            }
            Message::FlexShrinkChanged(value) => {
                self.flex_shrink = value;
            }
            Message::ContainerWidthChanged(width) => {
                self.container_width = width;
            }
            Message::ContainerHeightChanged(height) => {
                self.container_height = height;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let yellow_label = format!(
            "Flex grow:   {}\nFlex shrink: {:.1}",
            if self.flex_grow { "1.0" } else { "0.0" },
            self.flex_shrink
        );
        let yellow_box = flex(
            container(text(yellow_label).size(12).font(iced::Font::MONOSPACE))
                .padding(10)
                .style(style::yellow),
        )
        .grow(if self.flex_grow { 1.0 } else { 0.0 })
        .shrink(self.flex_shrink);

        let other_labels = format!("Flex grow:   0.0\nFlex shrink: 1.0");

        let mut boxes = (0..3)
            .map(|_| {
                flex(
                    container(
                        text(other_labels.clone())
                            .font(iced::Font::MONOSPACE)
                            .size(12),
                    )
                    .padding(10)
                    .style(style::gray),
                )
                .grow(0.0)
                .shrink(1.0)
            })
            .collect::<Vec<_>>();
        boxes.insert(0, yellow_box);

        let flex_container: Element<_> = match self.direction {
            Direction::Row => Row::from_vec(boxes)
                .spacing(5)
                .justify(JustifyContent::Center)
                .align(FlexAlignment::CenterFit)
                .into(),
            Direction::Column => Column::from_vec(boxes)
                .spacing(5)
                .justify(JustifyContent::Center)
                .align(FlexAlignment::CenterFit)
                .into(),
        };

        let controls = column![
            row![
                text("Direction: "),
                pick_list(
                    &Direction::ALL[..],
                    Some(self.direction),
                    Message::DirectionChanged
                )
            ]
            .align(Center),
            toggler(self.flex_grow)
                .label("Yellow box may grow")
                .on_toggle(Message::FlexGrowToggled),
            column![
                text(format!(
                    "Yellow box shrink factor: {:.1}",
                    self.flex_shrink
                )),
                slider(1.0..=8.0, self.flex_shrink, Message::FlexShrinkChanged)
                    .step(1.0)
            ],
            row![
                column![
                    text(format!(
                        "Container Height: {}",
                        self.container_height
                    )),
                    slider(
                        200..=400,
                        self.container_height,
                        Message::ContainerHeightChanged
                    )
                ]
                .width(Fill),
                column![
                    text(format!("Container Width: {}", self.container_width)),
                    slider(
                        500..=800,
                        self.container_width,
                        Message::ContainerWidthChanged
                    )
                ]
                .width(Fill)
            ]
            .spacing(10)
            .justify(JustifyContent::SpaceBetween),
        ]
        .spacing(20)
        .padding(20);

        column![
            iced::widget::center(
                container(flex_container)
                    .height(Fixed(self.container_height as f32))
                    .width(Fixed(self.container_width as f32))
                    .align_x(Center)
                    .align_y(Center)
                    .padding(20)
                    .style(container::bordered_box)
            ),
            controls
        ]
        .spacing(10)
        .padding(20)
        .into()
    }
}

mod style {
    use iced::widget::container;
    use iced::{color, Border, Color, Theme};

    pub fn gray(_: &Theme) -> container::Style {
        container::Style {
            background: Some(color!(0x1a1f23).into()),
            text_color: Some(Color::WHITE),
            border: Border {
                radius: 5.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn yellow(_: &Theme) -> container::Style {
        container::Style {
            background: Some(color!(0xffd500).into()),
            text_color: Some(Color::BLACK),
            border: Border {
                radius: 5.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
