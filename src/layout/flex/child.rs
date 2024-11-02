#![allow(unused_imports, dead_code)]
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, tree, Operation, Tree, Widget};
use iced::advanced::{overlay, renderer, Clipboard, Shell};
use iced::event::{self, Event};
use iced::{mouse, Transformation};
use iced::{
    Background, Border, Color, Element, Length, Padding, Pixels, Point,
    Rectangle, Size, Theme, Vector,
};

/// Properties controlling how a child behaves in a flex container
#[derive(Debug, Clone, Copy)]
pub struct FlexProperties {
    /// How much the item will grow relative to others
    pub grow: f32,
    /// How much the item will shrink relative to others
    pub shrink: f32,
    /// The hypothetical main axis size
    pub basis: Option<f32>,
    /// Whether the item can be stretched on cross axis
    pub can_stretch: bool,
}

impl Default for FlexProperties {
    fn default() -> Self {
        Self {
            grow: 0.0,
            shrink: 1.0,
            basis: None,
            can_stretch: false,
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
    /// Creates a new FlexChild with default properties
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            content: content.into(),
            properties: FlexProperties::default(),
        }
    }

    /// Sets how much this item will grow relative to other flex items
    pub fn grow(mut self, grow: f32) -> Self {
        self.properties.grow = grow;
        self
    }

    /// Sets how much this item will shrink relative to other flex items
    pub fn shrink(mut self, shrink: f32) -> Self {
        self.properties.shrink = shrink;
        self
    }

    /// Sets the basis (hypothetical main axis size) for this item
    pub fn basis(mut self, basis: f32) -> Self {
        self.properties.basis = Some(basis);
        self
    }

    /// Sets whether this item can stretch on the cross axis
    pub fn can_stretch(mut self, can_stretch: bool) -> Self {
        self.properties.can_stretch = can_stretch;
        self
    }

    /// Gets the flex properties
    pub fn properties(&self) -> &FlexProperties {
        &self.properties
    }

    /// Gets the inner content
    pub fn content(&self) -> &Element<'a, Message, Theme, Renderer> {
        &self.content
    }

    /// Gets the width of the child in `Length`
    pub(super) fn width(&self) -> Length {
        // If we have a basis, use that as a fixed width
        if let Some(basis) = self.properties.basis {
            Length::Fixed(basis)
        } else {
            self.content.as_widget().size().width
        }
    }

    /// Gets the height of the child in `Length`
    pub(super) fn height(&self) -> Length {
        self.content.as_widget().size().height
    }
}

impl<'a, Message, Theme, Renderer> FlexChild<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub(crate) fn state(&self) -> Tree {
        Tree::new(&self.content)
    }

    pub(crate) fn diff(&self, tree: &mut Tree) {
        // tree.diff(&self.content);
        self.content.as_widget().diff(tree);
    }

    /// Delegate layout to the inner content, letting the container of this
    /// [`FlexChild`] handle applying proper limits based on our properties
    pub(crate) fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let node = self.content.as_widget().layout(tree, renderer, limits);
        eprintln!("FlexChild layout: {:?}", node);
        node
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
        self.content
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport)
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
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
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
        self.content
            .as_widget()
            .operate(tree, layout, renderer, operation)
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
            tree,
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
        if let Some(layout_children) = layout.children().next() {
            self.content.as_widget().mouse_interaction(
                tree,
                layout_children,
                cursor,
                viewport,
                renderer,
            )
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, T, Message, Theme, Renderer> From<T>
    for FlexChild<'a, Message, Theme, Renderer>
where
    T: Into<Element<'a, Message, Theme, Renderer>>,
    Theme: iced::widget::container::Catalog + 'a,
    Renderer: renderer::Renderer,
{
    fn from(element: T) -> Self {
        Self::new(element)
    }
}
