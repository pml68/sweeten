use iced::advanced::{renderer, text};
use iced::Element;
use std::borrow::Borrow;

mod catalog {
    pub mod column {
        pub use crate::widget::column::Catalog;
    }
    pub mod row {
        pub use crate::widget::row::Catalog;
    }
}
pub use crate::widget::column::Column;
pub use crate::widget::mouse_area::MouseArea;
pub use crate::widget::overlay;
pub use crate::widget::pick_list::{self, PickList};
pub use crate::widget::row::Row;

/// A container intercepting mouse events.
pub fn mouse_area<'a, Message, Theme, Renderer>(
    widget: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> MouseArea<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    MouseArea::new(widget)
}

/// Pick lists display a dropdown list of selectable options, some of which
/// may be disabled.
pub fn pick_list<'a, T, L, V, Message, Theme, Renderer>(
    options: L,
    disabled: Option<impl Fn(&[T]) -> Vec<bool> + 'a>,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, V, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
    Theme: pick_list::Catalog + overlay::menu::Catalog,
    Renderer: text::Renderer,
{
    PickList::new(options, disabled, selected, on_selected)
}

#[macro_export]
macro_rules! column {
    () => (
        $crate::widget::Column::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::widget::Column::with_children([$(iced::Element::from($x)),+])
    );
}

pub fn column<'a, Message, Theme, Renderer>(
    children: impl IntoIterator<
        Item = impl Into<Element<'a, Message, Theme, Renderer>>,
    >,
) -> Column<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: iced::widget::container::Catalog + catalog::column::Catalog + 'a,
{
    Column::with_children(children)
}

#[macro_export]
macro_rules! row {
    () => (
        $crate::widget::Row::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::widget::Row::with_children([$(iced::Element::from($x)),+])
    );
}

pub fn row<'a, Message, Theme, Renderer>(
    children: impl IntoIterator<
        Item = impl Into<Element<'a, Message, Theme, Renderer>>,
    >,
) -> Row<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: iced::widget::container::Catalog + catalog::row::Catalog + 'a,
{
    Row::with_children(children)
}
