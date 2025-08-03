use iced::advanced::widget::Id;
use iced::keyboard::{key::Named, on_key_press, Key};
use iced::widget::{column, text};
use iced::{Center, Element, Subscription, Task};
use sweeten::widget::{button, operation};
fn main() -> iced::Result {
    iced::application("A cooler counter", Counter::update, Counter::view)
        .subscription(Counter::subscription)
        .run()
}

#[derive(Default)]
struct Counter {
    value: i64,
    focused_button: Option<Button>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Button {
    Increment,
    Decrement,
}

impl Button {
    fn as_str(&self) -> &str {
        match self {
            Self::Increment => "Increment",
            Self::Decrement => "Decrement",
        }
    }

    fn from_id(id: Id) -> Option<Self> {
        if id == Id::new("Increment") {
            Some(Self::Increment)
        } else if id == Id::new("Decrement") {
            Some(Self::Decrement)
        } else {
            None
        }
    }

    fn to_id(&self) -> Id {
        Id::new(self.as_str().to_string())
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    FocusNext,
    FocusPrevious,
    ButtonFocused(Button),
    ButtonBlurred(Button),
}

impl Counter {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::FocusNext => {
                return operation::focus_next(|id| {
                    Button::from_id(id).map_or(Task::none(), |button| {
                        Task::done(Message::ButtonFocused(button))
                    })
                })
            }
            Message::FocusPrevious => {
                return operation::focus_previous(|id| {
                    Button::from_id(id).map_or(Task::none(), |button| {
                        Task::done(Message::ButtonFocused(button))
                    })
                })
            }
            Message::ButtonFocused(button) => {
                println!("Button \"{}\" focused!", button.as_str());
                self.focused_button = Some(button);
            }
            Message::ButtonBlurred(button) => {
                if self.focused_button.is_some_and(|focused| focused == button)
                {
                    println!("Button \"{}\" blurred!", button.as_str());
                    self.focused_button = None;
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            button("Increment")
                .id(Button::Increment.to_id())
                .on_press(Message::Increment)
                .on_focus(Message::ButtonFocused(Button::Increment))
                .on_blur(Message::ButtonBlurred(Button::Increment)),
            text(self.value).size(50),
            button("Decrement")
                .id(Button::Decrement.to_id())
                .on_press(Message::Decrement)
                .on_focus(Message::ButtonFocused(Button::Decrement))
                .on_blur(Message::ButtonBlurred(Button::Decrement)),
        ]
        .padding(20)
        .align_x(Center)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        on_key_press(|key, modifiers| match key {
            Key::Named(Named::Tab) => {
                if modifiers.shift() {
                    Some(Message::FocusPrevious)
                } else {
                    Some(Message::FocusNext)
                }
            }
            _ => None,
        })
    }
}
