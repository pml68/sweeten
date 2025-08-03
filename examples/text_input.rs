use iced::advanced::widget;
use iced::widget::{center, column, container, row, text};
use iced::{Center, Element, Fill, Subscription, Task};

use sweeten::widget::{button, operation, text_input};

fn main() -> iced::Result {
    iced::application(
        "sweeten • iced text_input example",
        App::update,
        App::view,
    )
    .window(iced::window::Settings {
        min_size: Some([600.0, 400.0].into()),
        ..iced::window::Settings::default()
    })
    .subscription(App::subscription)
    .run_with(App::new)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Username,
    Password,
}

impl Field {
    fn as_str(&self) -> &'static str {
        match self {
            Field::Username => "username",
            Field::Password => "password",
        }
    }

    fn from_widget_id(id: widget::Id) -> Option<Self> {
        match id {
            id if id == Field::Username.id().into() => Some(Field::Username),
            id if id == Field::Password.id().into() => Some(Field::Password),
            _ => None,
        }
    }

    fn id(&self) -> text_input::Id {
        self.as_str().into()
    }

    fn placeholder(&self) -> &'static str {
        match self {
            Field::Username => "Enter username",
            Field::Password => "Enter password",
        }
    }

    fn validation_hint(&self) -> &'static str {
        match self {
            Field::Username => "Letters and numbers only",
            Field::Password => "Go to town, but min length is 12!",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    field: Field,
    value: String,
    error: Option<String>,
}

impl Input {
    fn new(field: Field) -> Self {
        Self {
            field,
            value: String::new(),
            error: None,
        }
    }

    fn field(&self) -> Field {
        self.field
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    fn with_value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    fn validate(mut self) -> Self {
        match self.field {
            Field::Username => {
                if self.value.is_empty() {
                    self.error = Some("Username is required".to_string());
                } else if !self.value.chars().all(|c| c.is_alphanumeric()) {
                    self.error = Some("Letters and numbers only".to_string());
                } else {
                    self.error = None;
                }
            }
            Field::Password => {
                if self.value.is_empty() {
                    self.error = Some("Password is required".to_string());
                } else if self.value.len() < 12 {
                    self.error = Some(
                        "Password must be at least 12 characters".to_string(),
                    );
                } else {
                    self.error = None;
                }
            }
        }
        self
    }
}

#[derive(Debug)]
struct App {
    username: Input,
    password: Input,
    focused_field: Option<Field>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(Field, String),
    InputFocused(Field),
    InputBlurred(Field),
    SubmitForm,
    FocusNext,
    FocusPrevious,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                username: Input::new(Field::Username),
                password: Input::new(Field::Password),
                focused_field: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(field, value) => match field {
                Field::Username => {
                    self.username =
                        self.username.clone().with_value(value).validate();
                }
                Field::Password => {
                    self.password =
                        self.password.clone().with_value(value).validate();
                }
            },
            Message::InputFocused(field) => {
                self.focused_field = Some(field);
            }
            Message::InputBlurred(field) => {
                if self.focused_field == Some(field) {
                    self.focused_field = None;
                }
            }
            Message::SubmitForm => {
                self.username = self.username.clone().validate();
                self.password = self.password.clone().validate();

                if self.form_is_valid() {
                    // Would submit the form here if this was a real app
                    println!("Form submitted successfully!");
                } else {
                    // Focus the first invalid field
                    let field_to_focus = if self.username.error().is_some() {
                        Field::Username
                    } else {
                        Field::Password
                    };

                    return text_input::focus(field_to_focus.id()).chain(
                        Task::done(Message::InputFocused(field_to_focus)),
                    );
                }
            }
            Message::FocusNext => {
                return operation::focus_next(|id| {
                    Field::from_widget_id(id).map_or_else(
                        || Task::none(),
                        |field| Task::done(Message::InputFocused(field)),
                    )
                });
            }
            Message::FocusPrevious => {
                return operation::focus_previous(|id| {
                    Field::from_widget_id(id).map_or_else(
                        || Task::none(),
                        |field| Task::done(Message::InputFocused(field)),
                    )
                });
            }
        }
        Task::none()
    }

    fn form_is_valid(&self) -> bool {
        !self.username.value().is_empty()
            && !self.password.value().is_empty()
            && self.username.error().is_none()
            && self.password.error().is_none()
    }

    fn view(&self) -> Element<'_, Message> {
        let create_field_view = |input: &Input| {
            let field = input.field();
            let value = input.value();
            let is_focused = self.focused_field == Some(field);

            let input_widget = text_input(field.placeholder(), value)
                .id(field.id())
                .on_input(move |text| Message::InputChanged(field, text))
                .on_focus(move |_| Message::InputFocused(field))
                .on_blur(Message::InputBlurred(field))
                .width(Fill)
                .secure(field == Field::Password);

            let status_text_content = if let Some(error) = input.error() {
                format!("Error: {}", error)
            } else if is_focused {
                field.validation_hint().to_string()
            } else {
                String::default()
            };

            let status_text = text(status_text_content).size(10.0).style(
                if input.error().is_some() {
                    text::danger
                } else {
                    text::primary
                },
            );

            column![
                text(field.as_str().to_uppercase()),
                input_widget,
                status_text
            ]
            .spacing(5)
        };

        let submit_button = button(text("Submit").center())
            .on_press_maybe(self.form_is_valid().then(|| Message::SubmitForm))
            .width(120);

        let form_status_content = if self.username.error().is_some()
            || self.password.error().is_some()
        {
            "Please fix the errors above"
        } else if self.form_is_valid() {
            "Form is valid!"
        } else {
            ""
        };

        let form_status =
            text(form_status_content).style(if self.form_is_valid() {
                text::success
            } else {
                text::danger
            });

        Element::from(
            center(
                column![
                    create_field_view(&self.username),
                    create_field_view(&self.password),
                    row![
                        form_status,
                        container(submit_button).align_right(Fill)
                    ]
                    .spacing(20)
                    .align_y(Center)
                ]
                .width(400)
                .align_x(Center)
                .spacing(20),
            )
            .padding(20),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        use iced::event::{self, Event};
        use iced::keyboard::{self, key::Named, Key};

        event::listen_with(|event, _, _| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modifiers,
                ..
            }) => match key {
                Key::Named(Named::Tab) => {
                    if modifiers.shift() {
                        Some(Message::FocusPrevious)
                    } else {
                        Some(Message::FocusNext)
                    }
                }
                Key::Named(Named::Enter) => Some(Message::SubmitForm),
                _ => None,
            },
            _ => None,
        })
    }
}
