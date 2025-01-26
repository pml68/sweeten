use iced::advanced::widget::operation::{Operation, Outcome};
use iced::advanced::widget::{operate, Id};
use iced::{Rectangle, Task};
use std::any::Any;

/// The internal state for tracking widget positions.
pub trait Position {
    /// Store the position of a child widget by index.
    fn set(&mut self, index: usize, bounds: Rectangle);

    /// Get the position of a child widget by index.
    fn get(&self, index: usize) -> Option<Rectangle>;

    /// Clear all stored positions.
    fn clear(&mut self);
}

// A concrete wrapper type that can be used for downcasting
pub struct PositionState {
    position: Box<dyn Position>,
}

impl PositionState {
    pub fn new<T: Position + 'static>(position: T) -> Self {
        Self {
            position: Box::new(position),
        }
    }
}

impl PositionState {
    pub fn as_position(&self) -> &dyn Position {
        &*self.position
    }

    pub fn as_position_mut(&mut self) -> &mut dyn Position {
        &mut *self.position
    }
}

/// Query to find the position of a specific child by index
pub struct Query {
    target_index: usize,
    found_bounds: Option<Rectangle>,
}

impl Query {
    pub fn new(index: usize) -> Self {
        Self {
            target_index: index,
            found_bounds: None,
        }
    }
}

impl Operation<Option<Rectangle>> for Query {
    fn container(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        operate_on_children: &mut dyn FnMut(
            &mut dyn Operation<Option<Rectangle>>,
        ),
    ) {
        // If we haven't found our target yet, keep searching children
        if self.found_bounds.is_none() {
            operate_on_children(self);
        }
    }

    fn custom(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        state: &mut dyn Any,
    ) {
        if self.found_bounds.is_none() {
            if let Some(position_state) = state.downcast_mut::<PositionState>()
            {
                self.found_bounds =
                    position_state.as_position().get(self.target_index);
            }
        }
    }

    fn finish(&self) -> Outcome<Option<Rectangle>> {
        Outcome::Some(self.found_bounds)
    }
}

/// Create a [`Task`](iced::Task) to find the position of a specific child by index
pub fn find_position(index: usize) -> Task<Option<Rectangle>> {
    operate(Query::new(index))
}
