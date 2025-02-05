use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Tree};
use iced::advanced::{overlay, renderer, Clipboard, Shell};
use iced::event::{self, Event};
use iced::{mouse, Element, Length, Rectangle, Vector};

use crate::layout::flex::Axis;

#[derive(Debug, Clone, Copy)]
pub struct AxisProperties {
    pub(crate) grow: f32,
    pub(crate) shrink: f32,
    pub(crate) basis: Option<f32>,
}

impl Default for AxisProperties {
    fn default() -> Self {
        Self {
            grow: 0.0,
            shrink: 1.0,
            basis: None,
        }
    }
}

pub struct FlexProperties {
    horizontal: AxisProperties,
    vertical: AxisProperties,
}

impl FlexProperties {
    pub(crate) fn main(&self, axis: Axis) -> &AxisProperties {
        match axis {
            Axis::Horizontal => &self.horizontal,
            Axis::Vertical => &self.vertical,
        }
    }
}

/// A wrapper around an Element that adds flex layout properties
pub struct FlexChild<'a, Message, Theme, Renderer = iced::Renderer>
where
    Renderer: renderer::Renderer,
{
    /// The wrapped element
    content: Element<'a, Message, Theme, Renderer>,
    /// Flex layout properties
    properties: FlexProperties,
}

impl<'a, Message, Theme, Renderer> FlexChild<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Creates a new FlexChild with properties derived from the element
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let content = content.into();
        let size = content.as_widget().size();

        // Initialize properties for each axis based on size hints
        let horizontal = AxisProperties {
            grow: if size.width.is_fill() { 1.0 } else { 0.0 },
            shrink: match size.width {
                Length::Fixed(_) => 0.0,
                Length::Shrink => 1.0,
                _ => {
                    if size.width.fluid() == Length::Shrink {
                        1.0
                    } else {
                        0.0
                    }
                }
            },
            basis: None,
        };

        let vertical = AxisProperties {
            grow: if size.height.is_fill() { 1.0 } else { 0.0 },
            shrink: match size.height {
                Length::Fixed(_) => 0.0,
                Length::Shrink => 1.0,
                _ => {
                    if size.height.fluid() == Length::Shrink {
                        1.0
                    } else {
                        0.0
                    }
                }
            },
            basis: None,
        };

        Self {
            content,
            properties: FlexProperties {
                horizontal,
                vertical,
            },
        }
    }

    /// Sets how much this item will grow horizontally
    pub fn grow_width(mut self, grow: f32) -> Self {
        self.properties.horizontal.grow = grow;
        self
    }

    /// Sets how much this item will grow vertically
    pub fn grow_height(mut self, grow: f32) -> Self {
        self.properties.vertical.grow = grow;
        self
    }

    /// Sets how much this item will shrink horizontally
    pub fn shrink_width(mut self, shrink: f32) -> Self {
        self.properties.horizontal.shrink = shrink;
        self
    }

    /// Sets how much this item will shrink vertically
    pub fn shrink_height(mut self, shrink: f32) -> Self {
        self.properties.vertical.shrink = shrink;
        self
    }

    /// Sets the basis for horizontal axis
    pub fn width_basis(mut self, basis: f32) -> Self {
        self.properties.horizontal.basis = Some(basis);
        self
    }

    /// Sets the basis for vertical axis
    pub fn height_basis(mut self, basis: f32) -> Self {
        self.properties.vertical.basis = Some(basis);
        self
    }

    /// Sets grow factor for both axes
    pub fn grow(self, grow: f32) -> Self {
        self.grow_width(grow).grow_height(grow)
    }

    /// Sets shrink factor for both axes
    pub fn shrink(self, shrink: f32) -> Self {
        self.shrink_width(shrink).shrink_height(shrink)
    }

    /// Gets the flex properties
    pub fn properties(&self) -> &FlexProperties {
        &self.properties
    }

    /// Gets the inner content
    pub fn content(&self) -> &Element<'a, Message, Theme, Renderer> {
        &self.content
    }
}

impl<'a, Message, Theme, Renderer> FlexChild<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(crate) fn state(&self) -> Tree {
        Tree {
            children: vec![Tree::new(&self.content)],
            ..Tree::empty()
        }
    }

    pub(crate) fn diff(&self, tree: &mut Tree) {
        tree.children[0].diff(&self.content);
    }

    pub(crate) fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size_hints = self.content.as_widget().size();

        // First get natural size with loose limits
        let natural_node = self.content.as_widget().layout(
            &mut tree.children[0],
            renderer,
            &limits.loose(),
        );

        // Create constrained limits based on natural size and size hints
        let min_width = match size_hints.width {
            Length::Shrink => natural_node.size().width,
            Length::Fixed(px) => px,
            _ => 0.0,
        };

        let min_height = match size_hints.height {
            Length::Shrink => natural_node.size().height,
            Length::Fixed(px) => px,
            _ => 0.0,
        };

        // Final layout with size constraints
        let constrained_limits =
            limits.min_width(min_width).min_height(min_height);

        self.content.as_widget().layout(
            &mut tree.children[0],
            renderer,
            &constrained_limits,
        )
    }

    /// Delegate drawing to the inner content
    pub(crate) fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    /// Delegate event handling to the inner content
    pub(crate) fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    /// Delegate widget operations to the inner content
    pub(crate) fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.content.as_widget().operate(
            &mut tree.children[0],
            layout,
            renderer,
            operation,
        )
    }

    /// Draw the overlay of the inner content
    pub(crate) fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            translation,
        )
    }

    /// Get the mouse interaction of the inner content
    pub(crate) fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }
}

impl<'a, Message, Theme, Renderer> FlexChild<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn from<T>(element: T) -> Self
    where
        T: Into<Element<'a, Message, Theme, Renderer>>,
    {
        Self::new(element)
    }
}

impl<'a, Message, Theme, Renderer> From<FlexChild<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(child: FlexChild<'a, Message, Theme, Renderer>) -> Self {
        child.content
    }
}
