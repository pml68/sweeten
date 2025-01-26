use iced::advanced::{renderer, text};
use iced::Element;
use std::borrow::Borrow;

pub mod mouse_area;
pub mod overlay;
pub mod pick_list;
pub mod row;

/// A container intercepting mouse events.
pub fn mouse_area<'a, Message, Theme, Renderer>(
    widget: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> mouse_area::MouseArea<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    mouse_area::MouseArea::new(widget)
}

/// Pick lists display a dropdown list of selectable options, some of which
/// may be disabled.
pub fn pick_list<'a, T, L, V, Message, Theme, Renderer>(
    options: L,
    disabled: Option<impl Fn(&[T]) -> Vec<bool> + 'a>,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> pick_list::PickList<'a, T, L, V, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone,
    Theme: pick_list::Catalog + overlay::menu::Catalog,
    Renderer: text::Renderer,
{
    pick_list::PickList::new(options, disabled, selected, on_selected)
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
    children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> row::Row<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: 'a,
{
    row::Row::with_children(children)
}
