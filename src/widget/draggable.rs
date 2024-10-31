use iced::Point;

#[derive(Debug, Clone)]
pub enum Action {
    Idle,
    Picking {
        index: usize,
        origin: Point,
    },
    Dragging {
        index: usize,
        origin: Point,
        last_cursor: Point,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum DropPosition {
    Before,
    Swap,
    After,
}

#[derive(Debug, Clone)]
pub enum DragEvent {
    Picked {
        index: usize,
    },
    Dropped {
        index: usize,
        target_index: usize,
        drop_position: DropPosition,
    },
    Canceled {
        index: usize,
    },
}
