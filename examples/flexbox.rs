use iced::widget::{
    column as iced_column, container, pick_list, row as iced_row, text,
};
use iced::Length::Fill;
use iced::{Center, Element, Task, Theme};

use sweeten::layout::flex::{FlexAlignment, FlexChild, JustifyContent};
use sweeten::widget::column::Column;
use sweeten::widget::draggable::{DragEvent, DropPosition};
use sweeten::widget::row::{row, Row};

pub fn main() -> iced::Result {
    iced::application(
        "sweetened iced - flex row and column",
        App::update,
        App::view,
    )
    .window(iced::window::Settings {
        size: (1000.0, 600.0).into(),
        min_size: Some((800.0, 550.0).into()),
        ..Default::default()
    })
    .theme(App::theme)
    .run_with(App::new)
}

#[derive(Default)]
struct App {
    elements: Vec<String>,
    mode: Mode,
    explain: bool,
    justify: Justify,
    align: Align,
}

#[derive(Debug, Clone)]
enum Message {
    Reorder(DragEvent),
    Mode(Mode),
    Justify(Justify),
    Align(Align),
    Explain(bool),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                elements: vec![
                    "Some rather\nlarge\nApple text".to_string(),
                    "Banana".to_string(),
                    "Cherry".to_string(),
                    "Date".to_string(),
                    "Elderberry".to_string(),
                ],
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        eprintln!("");
        match message {
            Message::Mode(mode) => {
                self.mode = mode;
            }
            Message::Justify(justify) => {
                self.justify = justify.into();
            }
            Message::Align(align) => {
                self.align = align.into();
            }
            Message::Explain(b) => {
                self.explain = b;
            }
            Message::Reorder(event) => {
                match event {
                    DragEvent::Picked { .. } => {
                        // Optionally handle pick event
                    }
                    DragEvent::Dropped {
                        index,
                        target_index,
                        drop_position,
                    } => {
                        // Update self.elements based on index, target_index, drop_position
                        match drop_position {
                            DropPosition::Before | DropPosition::After => {
                                if target_index != index
                                    && target_index != index + 1
                                {
                                    let item = self.elements.remove(index);
                                    let insert_index = if index < target_index {
                                        target_index - 1
                                    } else {
                                        target_index
                                    };
                                    self.elements.insert(insert_index, item);
                                }
                            }
                            DropPosition::Swap => {
                                if target_index != index {
                                    self.elements.swap(index, target_index);
                                }
                            }
                        }
                    }
                    DragEvent::Canceled { .. } => {
                        // Optionally handle cancel event
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let iced_items: Vec<Element<_>> = self
            .elements
            .iter()
            .map(|label| {
                container(text(label).center())
                    .style(container::rounded_box)
                    .padding(5)
                    .into()
            })
            .collect();

        let flex_items = self.elements.iter().enumerate().map(|(i, label)| {
            if i == 0 {
                pickme(label).can_stretch(false)
            } else {
                pickme(label)
            }
        });

        let iced_layout: Element<'_, Message> = match self.mode {
            Mode::Column => {
                let col = iced_column(iced_items)
                    .spacing(5)
                    .width(Fill)
                    .align_x(self.align);
                col.into()
            }
            Mode::Row => {
                let row = iced_row(iced_items)
                    .spacing(5)
                    .height(Fill)
                    .align_y(self.align);
                row.into()
            }
        };

        let flex_layout: Element<'_, Message> = match self.mode {
            Mode::Column => Column::with_flex_children(flex_items)
                .spacing(5)
                .width(Fill)
                .justify(self.justify)
                .align(self.align)
                .on_drag(Message::Reorder)
                .into(),
            Mode::Row => Row::with_flex_children(flex_items)
                .spacing(5)
                .height(Fill)
                .justify(self.justify)
                .align(self.align)
                .on_drag(Message::Reorder)
                .into(),
        };

        // Controls
        let justify_pick_list = iced_row![
            text("Justify: "),
            pick_list(&Justify::ALL[..], Some(&self.justify), Message::Justify)
        ]
        .align_y(Center);

        let align_pick_list = iced_row![
            text("Align: "),
            pick_list(&Align::ALL[..], Some(&self.align), Message::Align)
        ]
        .align_y(Center);

        let mode_pick_list = iced_row![
            text("Mode: "),
            pick_list(
                [Mode::Row, Mode::Column],
                Some(&self.mode),
                Message::Mode
            )
        ]
        .align_y(Center);

        let explain = iced::widget::checkbox("Explain", self.explain)
            .on_toggle(Message::Explain);

        // Apply explain if enabled
        let (iced_layout, flex_layout) = match self.explain {
            true => (
                iced_layout.explain(iced::color!(128, 0, 192)),
                flex_layout.explain(iced::color!(128, 0, 192)),
            ),
            false => (iced_layout, flex_layout),
        };

        // Here we give the `iced` layout the closest equivalent to the flexbox
        // properties. We use the `align` property to align the children within
        // the container, and the `justify` property to align the container
        let justify_y = iced::alignment::Vertical::from(self.justify);
        let justify_x = iced::alignment::Horizontal::from(self.justify);

        let align_x = iced::alignment::Horizontal::from(self.align);
        let align_y = iced::alignment::Vertical::from(self.align);
        let row_column_alignment = {
            match self.mode {
                Mode::Column => label("Column X", format!("{:?}", align_x)),
                Mode::Row => label("Row Y", format!("{:?}", align_y)),
            }
        };

        // Create the side-by-side containers
        let iced_container = iced_column![
            container(iced_layout)
                .align_x(justify_x)
                .align_y(justify_y)
                .width(Fill)
                .height(Fill)
                .padding(20)
                .style(style::bordered),
            iced_row![
                label(
                    "Container Align",
                    format!("{justify_y:?}/{justify_x:?}")
                ),
                label("Container Length", "Fill/Fill"),
                row_column_alignment,
            ]
            .align_y(Center)
            .spacing(20)
            .padding(10)
        ];

        let flex_container = iced_column![
            container(flex_layout)
                .align_x(justify_x)
                .align_y(justify_y)
                .width(Fill)
                .height(Fill)
                .padding(20)
                .style(style::bordered),
            iced_row![
                label("Justify", self.justify),
                label("Align", self.align),
            ]
            .align_y(Center)
            .spacing(20)
            .padding(10)
        ];

        // Layout controls and containers
        let controls: Vec<Element<'_, Message>> = vec![
            justify_pick_list.into(),
            align_pick_list.into(),
            mode_pick_list.into(),
            explain.into(),
        ];

        container(
            iced_column![
                row(controls).spacing(20),
                iced_row![
                    iced_column![
                        text("Standard Iced Layout").size(20),
                        iced_container
                    ]
                    .spacing(10)
                    .width(Fill),
                    iced_column![text("Flex Layout").size(20), flex_container]
                        .spacing(10)
                        .width(Fill),
                ]
                .spacing(20)
            ]
            .spacing(20)
            .padding(20),
        )
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightLight
    }
}

fn pickme(label: &str) -> FlexChild<'_, Message, Theme> {
    FlexChild::new(
        container(text(label).center())
            .style(container::rounded_box)
            .padding(5),
    )
}

fn label(label: &str, value: impl std::fmt::Display) -> Element<'_, Message> {
    text(format!("{}: {}", label, value)).size(11.0).into()
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Column => write!(f, "Column"),
            Mode::Row => write!(f, "Row"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Mode {
    Row,
    #[default]
    Column,
}

// In proper code, these would be unnecessary and you could just leverage
// flex::JustifyContent and flex::FlexAlignment directly. We use the below
// to help create pick lists and display their current values more easily.

#[derive(Default, Clone, Debug, PartialEq, Copy, Eq)]
enum Justify {
    #[default]
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Justify {
    const ALL: [Justify; 6] = [
        Justify::Start,
        Justify::End,
        Justify::Center,
        Justify::SpaceBetween,
        Justify::SpaceAround,
        Justify::SpaceEvenly,
    ];
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
enum Align {
    #[default]
    Start,
    End,
    Center,
    Stretch,
    Fit,
    CenterFit,
    EndFit,
}

impl Align {
    const ALL: [Align; 7] = [
        Align::Start,
        Align::End,
        Align::Center,
        Align::Stretch,
        Align::Fit,
        Align::CenterFit,
        Align::EndFit,
    ];
}

impl std::fmt::Display for Justify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Justify::Start => write!(f, "Start"),
            Justify::End => write!(f, "End"),
            Justify::Center => write!(f, "Center"),
            Justify::SpaceBetween => write!(f, "Space Between"),
            Justify::SpaceAround => write!(f, "Space Around"),
            Justify::SpaceEvenly => write!(f, "Space Evenly"),
        }
    }
}

impl std::fmt::Display for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Align::Start => write!(f, "Start"),
            Align::End => write!(f, "End"),
            Align::Center => write!(f, "Center"),
            Align::Stretch => write!(f, "Stretch"),
            Align::Fit => write!(f, "Fit"),
            Align::CenterFit => write!(f, "Center Fit"),
            Align::EndFit => write!(f, "End Fit"),
        }
    }
}

impl From<Justify> for JustifyContent {
    fn from(justify: Justify) -> Self {
        match justify {
            Justify::Start => JustifyContent::Start,
            Justify::End => JustifyContent::End,
            Justify::Center => JustifyContent::Center,
            Justify::SpaceBetween => JustifyContent::SpaceBetween,
            Justify::SpaceAround => JustifyContent::SpaceAround,
            Justify::SpaceEvenly => JustifyContent::SpaceEvenly,
        }
    }
}

impl From<Justify> for iced::Alignment {
    fn from(justify: Justify) -> Self {
        JustifyContent::from(justify).into()
    }
}

impl From<Justify> for iced::alignment::Horizontal {
    fn from(justify: Justify) -> Self {
        iced::Alignment::from(justify).into()
    }
}

impl From<Justify> for iced::alignment::Vertical {
    fn from(justify: Justify) -> Self {
        iced::Alignment::from(justify).into()
    }
}

impl From<Align> for FlexAlignment {
    fn from(align: Align) -> Self {
        match align {
            Align::Start => FlexAlignment::Start,
            Align::End => FlexAlignment::End,
            Align::Center => FlexAlignment::Center,
            Align::Stretch => FlexAlignment::Stretch,
            Align::Fit => FlexAlignment::Fit,
            Align::CenterFit => FlexAlignment::CenterFit,
            Align::EndFit => FlexAlignment::EndFit,
        }
    }
}

impl From<Align> for iced::Alignment {
    fn from(align: Align) -> Self {
        FlexAlignment::from(align).into()
    }
}

impl From<Align> for iced::alignment::Horizontal {
    fn from(align: Align) -> Self {
        iced::Alignment::from(align).into()
    }
}

impl From<Align> for iced::alignment::Vertical {
    fn from(align: Align) -> Self {
        iced::Alignment::from(align).into()
    }
}

mod style {
    use iced::widget::container;

    pub fn bordered(_theme: &super::Theme) -> container::Style {
        container::Style {
            border: iced::Border {
                color: iced::Color::BLACK.scale_alpha(0.2),
                width: 1.0,
                radius: 5.0.into(),
            },
            ..Default::default()
        }
    }
}
