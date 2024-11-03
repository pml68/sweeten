use iced::advanced::layout::{Limits, Node};
use iced::advanced::{renderer, widget};
use iced::{Alignment, Element, Length, Padding, Point, Size};

pub mod child;
pub use child::{FlexChild, FlexProperties};

/// Create a [`FlexChild`] with additional configuration options
pub fn flex<'a, E, Message, Theme, Renderer>(
    element: E,
) -> FlexChild<'a, Message, Theme, Renderer>
where
    E: Into<Element<'a, Message, Theme, Renderer>>,
    Renderer: renderer::Renderer,
{
    FlexChild::new(element)
}

/// The main axis of a flex layout.
#[derive(Debug)]
pub enum Axis {
    /// The horizontal axis
    Horizontal,
    /// The vertical axis
    Vertical,
}

impl Axis {
    fn main(&self, size: Size) -> f32 {
        match self {
            Axis::Horizontal => size.width,
            Axis::Vertical => size.height,
        }
    }

    fn cross(&self, size: Size) -> f32 {
        match self {
            Axis::Horizontal => size.height,
            Axis::Vertical => size.width,
        }
    }

    fn pack<T>(&self, main: T, cross: T) -> (T, T) {
        match self {
            Axis::Horizontal => (main, cross),
            Axis::Vertical => (cross, main),
        }
    }
}

/// Defines how items are distributed along the primary axis
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    /// Pack items at the start
    Start,
    /// Pack items at the end
    End,
    /// Center items
    Center,
    /// Distribute items with equal space between them
    SpaceBetween,
    /// Distribute items with equal space around them
    SpaceAround,
    /// Distribute items with equal space on both sides
    SpaceEvenly,
}

impl From<JustifyContent> for Alignment {
    fn from(justify: JustifyContent) -> Self {
        match justify {
            JustifyContent::End => Alignment::End,
            JustifyContent::Center => Alignment::Center,
            JustifyContent::Start | _ => Alignment::Start,
        }
    }
}

impl From<JustifyContent> for iced::alignment::Horizontal {
    fn from(justify: JustifyContent) -> Self {
        Alignment::from(justify).into()
    }
}

impl From<JustifyContent> for iced::alignment::Vertical {
    fn from(justify: JustifyContent) -> Self {
        Alignment::from(justify).into()
    }
}

/// Alignment options for flex layout
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexAlignment {
    /// Pack the item at the start
    Start,
    /// Pack the item at the end
    End,
    /// Center the item
    Center,
    /// Stretch to fill the container (default)
    Stretch,
    /// Make all items match the largest item's size, aligned at start
    Fit,
    /// Make all items match the largest item's size, centered
    CenterFit,
    /// Make all items match the largest item's size, aligned at end
    EndFit,
}

impl From<FlexAlignment> for iced::Alignment {
    fn from(alignment: FlexAlignment) -> Self {
        match alignment {
            FlexAlignment::Start | FlexAlignment::Fit => Alignment::Start,
            FlexAlignment::Center
            | FlexAlignment::CenterFit
            | FlexAlignment::Stretch => Alignment::Center,
            FlexAlignment::End | FlexAlignment::EndFit => Alignment::End,
        }
    }
}

impl From<Alignment> for FlexAlignment {
    fn from(alignment: Alignment) -> Self {
        match alignment {
            Alignment::Start => FlexAlignment::Start,
            Alignment::Center => FlexAlignment::Center,
            Alignment::End => FlexAlignment::End,
        }
    }
}

impl From<FlexAlignment> for iced::alignment::Horizontal {
    fn from(alignment: FlexAlignment) -> Self {
        Alignment::from(alignment).into()
    }
}

impl From<FlexAlignment> for iced::alignment::Vertical {
    fn from(alignment: FlexAlignment) -> Self {
        Alignment::from(alignment).into()
    }
}

/// Computes spacing for justify-content distribution
fn compute_spacing(
    justify: JustifyContent,
    available: f32,
    count: usize,
) -> (f32, f32) {
    match justify {
        JustifyContent::Start => (0.0, 0.0),
        JustifyContent::End => (available, 0.0),
        JustifyContent::Center => (available / 2.0, 0.0),
        JustifyContent::SpaceBetween => {
            if count <= 1 {
                (0.0, 0.0)
            } else {
                (0.0, available / (count - 1) as f32)
            }
        }
        JustifyContent::SpaceAround => {
            let space = available / count as f32;
            (space / 2.0, space)
        }
        JustifyContent::SpaceEvenly => {
            let space = available / (count + 1) as f32;
            (space, space)
        }
    }
}

pub fn resolve<Message, Theme, Renderer>(
    axis: Axis,
    renderer: &Renderer,
    limits: &Limits,
    width: Length,
    height: Length,
    padding: Padding,
    spacing: f32,
    justify_content: JustifyContent,
    align_items: FlexAlignment,
    items: &[FlexChild<'_, Message, Theme, Renderer>],
    trees: &mut [widget::Tree],
) -> Node
where
    Renderer: renderer::Renderer,
{
    if items.is_empty() {
        let size = limits.resolve(width, height, Size::ZERO);
        return Node::new(size);
    }

    // Apply container limits and padding
    let limits = limits.width(width).height(height).shrink(padding);
    let total_spacing = spacing * items.len().saturating_sub(1) as f32;

    // First pass: Calculate natural sizes and collect flex information
    let mut total_basis = 0.0;
    let mut total_grow = 0.0;
    let mut total_shrink = 0.0;
    let mut nodes = Vec::with_capacity(items.len());
    let mut natural_cross_max: f32 = 0.0;

    // First layout pass - get natural sizes
    for (child, tree) in items.iter().zip(trees.iter_mut()) {
        let properties = child.properties();
        let content = child.content().as_widget();
        let content_size = content.size();

        // Check if this item should stretch based on its Length properties
        let should_stretch = match axis {
            Axis::Horizontal => content_size.height.fluid().is_fill(),
            Axis::Vertical => content_size.width.fluid().is_fill(),
        };

        // Calculate basis
        let basis = properties.basis.unwrap_or_else(|| match axis {
            Axis::Horizontal => match content_size.width {
                Length::Fixed(px) => px,
                _ => 0.0,
            },
            Axis::Vertical => match content_size.height {
                Length::Fixed(px) => px,
                _ => 0.0,
            },
        });

        total_basis += basis;
        total_grow += properties.grow;
        total_shrink += properties.shrink;

        // Initial layout with natural size
        let child_limits = Limits::new(Size::ZERO, limits.max());
        let node = content.layout(tree, renderer, &child_limits);
        natural_cross_max = natural_cross_max.max(axis.cross(node.size()));
        nodes.push((node, properties, should_stretch));
    }

    // Determine container cross size based on content and container properties
    let container_cross = match axis {
        Axis::Horizontal => match height {
            Length::Fill | Length::FillPortion(_) => axis.cross(limits.max()),
            _ => natural_cross_max,
        },
        Axis::Vertical => match width {
            Length::Fill | Length::FillPortion(_) => axis.cross(limits.max()),
            _ => natural_cross_max,
        },
    };

    // Calculate available space for grow/shrink
    let container_main = axis.main(limits.max());
    let available_space = container_main - total_spacing - total_basis;
    let is_growing = available_space > 0.0;

    // Second pass: Apply flex properties and layout with final sizes
    let mut final_nodes = Vec::with_capacity(items.len());

    for ((child, tree), (mut node, properties, should_stretch)) in
        items.iter().zip(trees).zip(nodes)
    {
        let content = child.content().as_widget();
        let mut main_size = axis.main(node.size());

        // Apply growth/shrink
        if is_growing && properties.grow > 0.0 && total_grow > 0.0 {
            main_size += available_space * (properties.grow / total_grow);
        } else if !is_growing && properties.shrink > 0.0 && total_shrink > 0.0 {
            let shrink_amount = (-available_space * properties.shrink
                / total_shrink)
                .min(main_size);
            main_size -= shrink_amount;
        }

        // Determine cross size based on alignment and properties
        let should_stretch =
            should_stretch || align_items == FlexAlignment::Stretch;
        let cross_size = match align_items {
            FlexAlignment::Stretch if should_stretch => container_cross,
            FlexAlignment::Fit
            | FlexAlignment::CenterFit
            | FlexAlignment::EndFit => natural_cross_max,
            _ => axis.cross(node.size()),
        };

        // Create limits for final layout
        let (width, height) = axis.pack(main_size, cross_size);
        let child_limits = match align_items {
            FlexAlignment::Stretch if should_stretch => {
                // Force the cross size for stretching items
                let min_size = axis.pack(0.0, cross_size);
                Limits::new(
                    Size::new(min_size.0, min_size.1),
                    Size::new(width, height),
                )
                .width(content.size().width)
                .height(content.size().height)
            }
            FlexAlignment::Fit
            | FlexAlignment::CenterFit
            | FlexAlignment::EndFit => {
                // Force all items to the same cross size
                let min_size = axis.pack(0.0, cross_size);
                Limits::new(
                    Size::new(min_size.0, min_size.1),
                    Size::new(width, height),
                )
            }
            _ => Limits::new(Size::ZERO, Size::new(width, height)),
        };

        node = content.layout(tree, renderer, &child_limits);
        final_nodes.push(node);
    }

    // Calculate content size
    let total_main = final_nodes
        .iter()
        .fold(0.0, |acc, node| acc + axis.main(node.size()))
        + total_spacing;

    // Determine if we need full width for spacing
    let needs_full_main = matches!(
        justify_content,
        JustifyContent::SpaceBetween
            | JustifyContent::SpaceAround
            | JustifyContent::SpaceEvenly
    );

    // Calculate final container size
    let final_main = if needs_full_main {
        // Use full available space for Space* variants
        axis.main(limits.max())
    } else {
        // Use content size for Start/Center/End
        total_main + padding.horizontal()
    };
    let final_cross = container_cross + padding.vertical();

    // Calculate spacing for items within the container
    let available_space = if needs_full_main {
        final_main - total_main - padding.horizontal()
    } else {
        0.0
    };

    let (item_initial_offset, item_spacing) =
        compute_spacing(justify_content, available_space, final_nodes.len());

    // Position nodes within container bounds
    let mut main = padding.left + item_initial_offset;

    for (i, node) in final_nodes.iter_mut().enumerate() {
        if i > 0 {
            main += spacing + item_spacing;
        }

        let cross_offset = match align_items {
            FlexAlignment::End | FlexAlignment::EndFit => {
                padding.top + container_cross - axis.cross(node.size())
            }
            FlexAlignment::Center | FlexAlignment::CenterFit => {
                padding.top + (container_cross - axis.cross(node.size())) / 2.0
            }
            _ => padding.top,
        };

        let (x, y) = axis.pack(main, cross_offset);
        node.move_to_mut(Point::new(x, y));

        main += axis.main(node.size());
    }

    // Create container with proper size
    let container_size = axis.pack(final_main, final_cross);
    let intrinsic_size = Size::new(container_size.0, container_size.1);

    // Resolve final size based on length constraints
    let size = match axis {
        Axis::Horizontal => match width {
            Length::Shrink if !needs_full_main => {
                // Only shrink if not using Space* variants
                limits.resolve(
                    Length::Fixed(final_main),
                    height,
                    intrinsic_size,
                )
            }
            _ => limits.resolve(width, height, intrinsic_size),
        },
        Axis::Vertical => match height {
            Length::Shrink if !needs_full_main => {
                // Only shrink if not using Space* variants
                limits.resolve(width, Length::Fixed(final_main), intrinsic_size)
            }
            _ => limits.resolve(width, height, intrinsic_size),
        },
    };

    // Calculate offset (only for non-Space* variants)
    let offset = if !needs_full_main {
        match justify_content {
            JustifyContent::Start => 0.0,
            JustifyContent::End => axis.main(limits.max()) - final_main,
            JustifyContent::Center => {
                (axis.main(limits.max()) - final_main) / 2.0
            }
            _ => 0.0,
        }
    } else {
        0.0
    };

    // Create final node with appropriate offset
    let (offset_x, offset_y) = axis.pack(offset, 0.0);
    let mut container = Node::with_children(size, final_nodes);
    container.move_to_mut(Point::new(offset_x, offset_y));

    container
}
