use iced::widget::{
    column as iced_column, container, pick_list, row as iced_row, text,
};
use iced::Length::Fill;
use iced::{Center, Element, Task, Theme};

use sweeten::layout::flex::JustifyContent;
use sweeten::widget::draggable::{DragEvent, DropPosition};
use sweeten::widget::{column, row};

pub fn main() -> iced::Result {
    iced::application(
        "sweetened iced - flex row and column",
        App::update,
        App::view,
    )
    .window(iced::window::Settings {
        size: iced::Size::new(600.0, 600.0),
        ..Default::default()
    })
    .theme(App::theme)
    .run_with(App::new)
}

#[derive(Default)]
struct App {
    elements: Vec<String>,
    mode: Mode,
    justify: Justify,
}

#[derive(Debug, Clone, Default, PartialEq)]
enum Mode {
    #[default]
    Row,
    Column,
}

#[derive(Debug, Clone)]
enum Message {
    Reorder(DragEvent),
    SwitchMode(Mode),
    SwitchJustify(Justify),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                elements: vec![
                    "Apple".to_string(),
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
        match message {
            Message::SwitchMode(mode) => {
                self.mode = mode;
            }
            Message::SwitchJustify(justify) => {
                self.justify = justify.into();
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
        let items = self.elements.iter().map(|label| pickme(label));
        let drag: Element<'_, Message> = match self.mode {
            Mode::Column => column(items.collect::<Vec<_>>())
                .spacing(5)
                .height(Fill)
                .justify(&self.justify)
                // For the column example only, set the deadband_zone to zero
                .deadband_zone(0.0)
                .on_drag(Message::Reorder)
                .align_x(Center)
                .into(),
            Mode::Row => row(items.collect::<Vec<_>>())
                .spacing(5)
                .width(Fill)
                .justify(&self.justify)
                .align_y(Center)
                .on_drag(Message::Reorder)
                .into(),
        };

        let justify_pick_list = iced_row![
            text("Justify: "),
            pick_list(
                &Justify::ALL[..],
                Some(&self.justify),
                Message::SwitchJustify,
            )
        ]
        .align_y(Center)
        .into();

        let mode_pick_list = iced_row![
            text("Mode: "),
            pick_list(
                [Mode::Row, Mode::Column],
                Some(&self.mode),
                Message::SwitchMode,
            )
        ]
        .align_y(Center)
        .into();

        container(
            iced_column![
                row([justify_pick_list, mode_pick_list])
                    .justify(&Justify::SpaceBetween),
                // container(drag.explain(iced::color!(223, 66, 200)))
                container(drag)
                    .padding(0)
                    .width(Fill)
                    .height(Fill)
                    .align_x(Center)
                    .align_y(Center)
                    .style(style::bordered)
            ]
            .align_x(Center)
            .spacing(5),
        )
        .padding(20)
        .height(Fill)
        .width(Fill)
        .align_y(Center)
        .align_x(Center)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightLight
    }
}

fn pickme(label: &str) -> Element<'_, Message> {
    container(text(label))
        .style(container::rounded_box)
        .padding(5)
        .into()
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Column => write!(f, "Column"),
            Mode::Row => write!(f, "Row"),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
enum Justify {
    Start,
    End,
    #[default]
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

impl From<&Justify> for JustifyContent {
    fn from(justify: &Justify) -> Self {
        justify.clone().into()
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
