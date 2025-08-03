use iced::advanced::widget::{operate, operation::focusable, Id};
use iced::Task;

/// Produces a [`Task`] that focuses the next focusable widget
/// and then applies the provided function to create a resulting task.
pub fn focus_next<T, F>(f: F) -> Task<T>
where
    T: Send + 'static,
    F: Send + Sync + Fn(Id) -> Task<T> + 'static,
{
    operate(focusable::focus_next())
        .chain(operate(focusable::find_focused()).then(move |id| f(id)))
}

/// Produces a [`Task`] that focuses the previous focusable widget
/// and then applies the provided function to create a resulting task.
pub fn focus_previous<T, F>(f: F) -> Task<T>
where
    T: Send + 'static,
    F: Send + Sync + Fn(Id) -> Task<T> + 'static,
{
    operate(focusable::focus_previous())
        .chain(operate(focusable::find_focused()).then(move |id| f(id)))
}
