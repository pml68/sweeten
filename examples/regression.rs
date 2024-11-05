use iced::widget::{button, column as iced_column, container, row as iced_row};
use iced::{Element, Theme};

use sweeten::layout::{FlexAlignment, JustifyContent};
use sweeten::{column, row};

pub fn main() -> iced::Result {
    iced::application(
        "sweetened iced - flexbox benchmark",
        App::update,
        App::view,
    )
    .theme(|_| Theme::TokyoNightLight)
    .window(iced::window::Settings {
        size: (800.0, 600.0).into(),
        ..Default::default()
    })
    .run()
}

#[derive(Default)]
pub struct App {
    mode: Mode,
}

#[derive(Debug, Default, Clone)]
enum Mode {
    #[default]
    Sweetened,
    Iced,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleMode,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleMode => {
                *self = App {
                    mode: match self.mode {
                        Mode::Sweetened => Mode::Iced,
                        Mode::Iced => Mode::Sweetened,
                    },
                };
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            button(match self.mode {
                Mode::Sweetened => "Switch to Iced",
                Mode::Iced => "Switch to Sweetened",
            })
            .on_press(Message::ToggleMode),
            gen::view(&self.mode),
        ]
        .justify(JustifyContent::SpaceBetween)
        .align(FlexAlignment::Stretch)
        .into()
    }
}

mod gen {
    use super::*;
    use iced::Element;
    use iced::Length::{self, Fill, Fixed};
    use rand::Rng;

    // Define our layout tree structure
    #[derive(Clone)]
    enum Layout {
        Container {
            value: usize,
            width: Length,
            height: Length,
        },
        Row {
            children: Vec<Layout>,
            config: LayoutConfig,
        },
        Column {
            children: Vec<Layout>,
            config: LayoutConfig,
        },
    }

    #[derive(Clone)]
    struct LayoutConfig {
        width: Length,
        height: Length,
        spacing: u16,
        padding: u16,
        justify: Option<JustifyContent>,
        align: Option<FlexAlignment>,
    }

    impl Default for LayoutConfig {
        fn default() -> Self {
            Self {
                width: Fill,
                height: Fill,
                spacing: 5,
                padding: 5,
                justify: Some(JustifyContent::SpaceBetween),
                align: Some(FlexAlignment::CenterFit),
            }
        }
    }

    impl Layout {
        fn render(&self, mode: &Mode) -> Element<'static, Message> {
            match self {
                Layout::Container {
                    value,
                    width,
                    height,
                } => container(iced::widget::text(value))
                    .width(*width)
                    .height(*height)
                    .padding(5)
                    .style(style::next)
                    .into(),
                Layout::Row { children, config } => {
                    match mode {
                        Mode::Sweetened => {
                            let children = children
                                .iter()
                                .map(|child| child.render(&mode));
                            let mut row = row(children).spacing(config.spacing);

                            if let Some(justify) = &config.justify {
                                row = row.justify(*justify);
                            }
                            if let Some(align) = &config.align {
                                row = row.align(*align);
                            }

                            container(row)
                                .width(config.width)
                                .height(config.height)
                                .padding(config.padding)
                                .style(style::next)
                                .into()
                        }
                        Mode::Iced => {
                            let children =
                                children.iter().map(|child| child.render(mode));
                            let mut row =
                                iced_row(children).spacing(config.spacing);

                            // Convert sweeten alignment to iced alignment
                            if let Some(align) = &config.align {
                                row = row.align_y(
                                    iced::alignment::Vertical::from(*align),
                                )
                            }

                            container(row)
                                .width(config.width)
                                .height(config.height)
                                .padding(config.padding)
                                .style(style::next)
                                .into()
                        }
                    }
                }
                Layout::Column { children, config } => {
                    match mode {
                        Mode::Sweetened => {
                            let children =
                                children.iter().map(|child| child.render(mode));
                            let mut col =
                                column(children).spacing(config.spacing);

                            if let Some(justify) = &config.justify {
                                col = col.justify(*justify);
                            }
                            if let Some(align) = &config.align {
                                col = col.align(*align);
                            }

                            container(col)
                                .width(config.width)
                                .height(config.height)
                                .padding(config.padding)
                                .style(style::next)
                                .into()
                        }
                        Mode::Iced => {
                            let children =
                                children.iter().map(|child| child.render(mode));
                            let mut col =
                                iced_column(children).spacing(config.spacing);

                            // Convert sweeten alignment to iced alignment
                            if let Some(align) = &config.align {
                                col = col.align_x(
                                    iced::alignment::Horizontal::from(*align),
                                )
                            }

                            container(col)
                                .width(config.width)
                                .height(config.height)
                                .padding(config.padding)
                                .style(style::next)
                                .into()
                        }
                    }
                }
            }
        }

        fn generate(depth: u32, max_children: usize, min_size: f32) -> Self {
            let mut rng = rand::thread_rng();

            if depth == 0 || min_size < 50.0 {
                Layout::Container {
                    value: if rng.gen_bool(0.5) { 0 } else { 1 },
                    width: if rng.gen_bool(0.5) {
                        Fixed(min_size)
                    } else {
                        Fill
                    },
                    height: if rng.gen_bool(0.5) {
                        Fixed(min_size)
                    } else {
                        Fill
                    },
                }
            } else {
                let num_children = rng.gen_range(2..=max_children);
                let children = (0..num_children)
                    .map(|_| {
                        Self::generate(
                            depth - 1,
                            max_children,
                            min_size / num_children as f32,
                        )
                    })
                    .collect();

                let config = LayoutConfig {
                    width: if rng.gen_bool(0.8) {
                        Fill
                    } else {
                        Fixed(min_size)
                    },
                    height: if rng.gen_bool(0.8) {
                        Fill
                    } else {
                        Fixed(min_size)
                    },
                    justify: Some(match rng.gen_range(0..4) {
                        0 => JustifyContent::Start,
                        1 => JustifyContent::Center,
                        2 => JustifyContent::SpaceBetween,
                        _ => JustifyContent::SpaceEvenly,
                    }),
                    ..Default::default()
                };

                if rng.gen_bool(0.5) {
                    Layout::Row { children, config }
                } else {
                    Layout::Column { children, config }
                }
            }
        }
    }

    pub fn view(mode: &Mode) -> Element<'static, Message> {
        let layout = Layout::generate(7, 4, 800.0);

        container(layout.render(mode))
            .width(Fill)
            .height(Fill)
            .padding(10)
            .style(container::bordered_box)
            .into()
    }
}

mod style {
    use iced::widget::container;
    use iced::Color;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static COLOR_INDEX: AtomicUsize = AtomicUsize::new(0);

    const COLORS: [Color; 8] = [
        Color::from_rgb(0.95, 0.80, 0.80), // Light Red
        Color::from_rgb(0.80, 0.95, 0.80), // Light Green
        Color::from_rgb(0.80, 0.80, 0.95), // Light Blue
        Color::from_rgb(0.95, 0.95, 0.80), // Light Yellow
        Color::from_rgb(0.95, 0.80, 0.95), // Light Purple
        Color::from_rgb(0.80, 0.95, 0.95), // Light Cyan
        Color::from_rgb(0.90, 0.85, 0.80), // Light Orange
        Color::from_rgb(0.85, 0.85, 0.85), // Light Gray
    ];

    pub fn next(_theme: &iced::Theme) -> container::Style {
        // Get next color and increment counter atomically
        let index = COLOR_INDEX.fetch_add(1, Ordering::Relaxed) % COLORS.len();
        let color = COLORS[index];

        container::Style {
            background: Some(color.into()),
            border: iced::Border {
                width: 1.0,
                color: Color::BLACK,
                radius: 5.0.into(),
            },
            ..Default::default()
        }
    }
}
