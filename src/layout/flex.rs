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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
            JustifyContent::Start => Alignment::Start,
            JustifyContent::SpaceAround => Alignment::Start,
            JustifyContent::SpaceBetween => Alignment::Start,
            JustifyContent::SpaceEvenly => Alignment::Start,
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
            FlexAlignment::Start => Alignment::Start,
            FlexAlignment::Fit => Alignment::Start,
            FlexAlignment::Center => Alignment::Center,
            FlexAlignment::CenterFit => Alignment::Center,
            FlexAlignment::Stretch => Alignment::Center,
            FlexAlignment::End => Alignment::End,
            FlexAlignment::EndFit => Alignment::End,
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

/// Computes the flexbox layout with the given axis and limits, applying spacing,
/// main axis justification and cross axis alignment based on the container's
/// dimensions and properties as well as the children's sizes and properties.
///
/// It returns a new layout [`Node`].
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

    // Keep original limits for final container sizing
    let original_limits = limits.width(width).height(height);

    // Create shrunk limits for children
    let limits = original_limits.clone().shrink(padding);
    let total_spacing = spacing * items.len().saturating_sub(1) as f32;

    // First pass: Calculate natural sizes and collect flex information
    let mut total_basis = 0.0;
    let mut total_grow = 0.0;
    let mut total_shrink = 0.0;
    let mut total_fill_grow = 0.0;
    let mut fill_items_count = 0;
    let mut nodes = Vec::with_capacity(items.len());
    let mut natural_cross_max: f32 = 0.0;

    // First layout pass - get natural sizes and flex info
    for (child, tree) in items.iter().zip(trees.iter_mut()) {
        let main_props = child.properties().main(axis);
        let content_size = child.content().as_widget().size();

        // Check if this item has Fill in main axis
        let is_fill = match axis {
            Axis::Horizontal => content_size.width.is_fill(),
            Axis::Vertical => content_size.height.is_fill(),
        };

        // Get initial natural size with loose limits
        let child_limits = limits.loose();
        let node = child.content().as_widget().layout(
            &mut tree.children[0],
            renderer,
            &child_limits,
        );
        let natural_size = node.size();

        // For Fill items, don't add their full natural size to total_basis
        // Instead, track their grow factors separately
        if is_fill {
            total_fill_grow += main_props.grow;
            fill_items_count += 1;
        } else {
            let basis =
                main_props.basis.unwrap_or_else(|| axis.main(natural_size));
            total_basis += basis;
        }

        total_grow += main_props.grow;
        total_shrink += main_props.shrink;
        natural_cross_max = natural_cross_max.max(axis.cross(natural_size));

        nodes.push((node, natural_size, is_fill));
    }

    // Determine container dimensions
    let should_fill_main = match axis {
        Axis::Horizontal => width.is_fill(),
        Axis::Vertical => height.is_fill(),
    };

    let container_main = axis.main(limits.max());
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

    // Calculate space available for growth, excluding Fill items from initial content_size
    let content_size = total_basis + total_spacing;
    let fill_space = if fill_items_count > 0 {
        (container_main - content_size).max(0.0)
    } else {
        0.0
    };

    let remaining_space = container_main - content_size - fill_space;
    let is_growing = remaining_space > 0.0 && total_grow > 0.0;

    // Second pass: Apply flex properties and layout with final sizes
    let mut final_nodes = Vec::with_capacity(items.len());

    for ((child, tree), (_, natural_size, is_fill)) in
        items.iter().zip(trees).zip(nodes.clone())
    {
        let main_props = child.properties().main(axis);
        let mut main_size = axis.main(natural_size);

        if is_fill {
            // Distribute fill_space among Fill items according to their grow factors
            // main_size =
            //     (fill_space * main_props.grow / total_fill_grow).max(0.0);
            main_size = if main_props.grow > 0.0 {
                (fill_space * main_props.grow / total_fill_grow).max(0.0)
            } else {
                // Use our pre-counted fill_items_count
                fill_space / fill_items_count as f32
            };
        } else if is_growing {
            // Regular growing for non-Fill items
            let grow_unit = remaining_space / total_grow;
            main_size += main_props.grow * grow_unit;
        } else if main_props.shrink > 0.0
            && total_shrink > 0.0
            && remaining_space < 0.0
        {
            // Shrinking with weighted factors
            let shrink_weight = main_props.shrink * main_size;
            let total_weighted_shrink = items
                .iter()
                .zip(&nodes)
                .map(|(child, (_, size, _))| {
                    child.properties().main(axis).shrink * axis.main(*size)
                })
                .sum::<f32>();

            if total_weighted_shrink > 0.0 {
                let deficit = -remaining_space;
                let shrink_ratio = shrink_weight / total_weighted_shrink;
                main_size = (main_size - deficit * shrink_ratio).max(0.0);
            }
        }

        // Handle cross-axis sizing and stretching
        let should_stretch = match axis {
            Axis::Horizontal => {
                child.content().as_widget().size().height.is_fill()
                    || align_items == FlexAlignment::Stretch
            }
            Axis::Vertical => {
                child.content().as_widget().size().width.is_fill()
                    || align_items == FlexAlignment::Stretch
            }
        };

        let cross_size = match align_items {
            FlexAlignment::Stretch if should_stretch => container_cross,
            FlexAlignment::Fit
            | FlexAlignment::CenterFit
            | FlexAlignment::EndFit => natural_cross_max,
            _ => axis.cross(natural_size),
        };

        // Create limits for final layout
        let (width, height) = axis.pack(main_size, cross_size);
        let child_limits = match align_items {
            FlexAlignment::Stretch if should_stretch => {
                let (min_width, min_height) =
                    axis.pack(main_size, container_cross);
                let (max_width, max_height) =
                    axis.pack(main_size, container_cross);
                Limits::new(
                    Size::new(min_width, min_height),
                    Size::new(max_width, max_height),
                )
            }
            FlexAlignment::Fit
            | FlexAlignment::CenterFit
            | FlexAlignment::EndFit => {
                let min_size = axis.pack(main_size, cross_size);
                Limits::new(
                    Size::new(min_size.0, min_size.1),
                    Size::new(width, height),
                )
            }
            _ => {
                let min_main = axis.pack(main_size, 0.0);
                Limits::new(
                    Size::new(min_main.0, min_main.1),
                    Size::new(width, height),
                )
            }
        };

        let node = child.content().as_widget().layout(
            &mut tree.children[0],
            renderer,
            &child_limits,
        );

        final_nodes.push(node);
    }

    // Calculate final container dimensions
    let total_main = final_nodes
        .iter()
        .fold(0.0, |acc, node| acc + axis.main(node.size()))
        + total_spacing;

    let final_main = if should_fill_main
        || matches!(
            justify_content,
            JustifyContent::SpaceBetween
                | JustifyContent::SpaceAround
                | JustifyContent::SpaceEvenly
        ) {
        container_main
    } else {
        total_main.min(container_main)
    };

    // Calculate justify spacing
    let justify_space = if should_fill_main
        || !matches!(justify_content, JustifyContent::Start)
    {
        (final_main - total_main).max(0.0)
    } else {
        0.0
    };

    let (initial_offset, item_spacing) =
        compute_spacing(justify_content, justify_space, final_nodes.len());

    // Position nodes
    let mut main = padding.left + initial_offset;
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

    // Create final container with padding
    let container_size = axis.pack(final_main, container_cross);
    let size = original_limits.resolve(
        width,
        height,
        Size::new(
            container_size.0 + padding.horizontal(),
            container_size.1 + padding.vertical(),
        ),
    );

    Node::with_children(size, final_nodes)
}
