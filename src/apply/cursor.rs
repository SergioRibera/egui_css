use egui::{CursorIcon, Visuals};

pub(super) fn apply_cursor(s: &mut Visuals, cursor: cssengine::CursorIcon) {
    let cursor = match cursor {
        cssengine::CursorIcon::Default => CursorIcon::Default,
        cssengine::CursorIcon::None => CursorIcon::None,
        cssengine::CursorIcon::ContextMenu => CursorIcon::ContextMenu,
        cssengine::CursorIcon::Help => CursorIcon::Help,
        cssengine::CursorIcon::PointingHand => CursorIcon::PointingHand,
        cssengine::CursorIcon::Progress => CursorIcon::Progress,
        cssengine::CursorIcon::Wait => CursorIcon::Wait,
        cssengine::CursorIcon::Cell => CursorIcon::Cell,
        cssengine::CursorIcon::Crosshair => CursorIcon::Crosshair,
        cssengine::CursorIcon::Text => CursorIcon::Text,
        cssengine::CursorIcon::VerticalText => CursorIcon::VerticalText,
        cssengine::CursorIcon::Alias => CursorIcon::Alias,
        cssengine::CursorIcon::Copy => CursorIcon::Copy,
        cssengine::CursorIcon::Move => CursorIcon::Move,
        cssengine::CursorIcon::NoDrop => CursorIcon::NoDrop,
        cssengine::CursorIcon::NotAllowed => CursorIcon::NotAllowed,
        cssengine::CursorIcon::Grab => CursorIcon::Grab,
        cssengine::CursorIcon::Grabbing => CursorIcon::Grabbing,
        cssengine::CursorIcon::AllScroll => CursorIcon::AllScroll,
        cssengine::CursorIcon::ResizeHorizontal => CursorIcon::ResizeHorizontal,
        cssengine::CursorIcon::ResizeNeSw => CursorIcon::ResizeNeSw,
        cssengine::CursorIcon::ResizeNwSe => CursorIcon::ResizeNwSe,
        cssengine::CursorIcon::ResizeVertical => CursorIcon::ResizeVertical,
        cssengine::CursorIcon::ResizeEast => CursorIcon::ResizeEast,
        cssengine::CursorIcon::ResizeSouthEast => CursorIcon::ResizeSouthEast,
        cssengine::CursorIcon::ResizeSouth => CursorIcon::ResizeSouth,
        cssengine::CursorIcon::ResizeSouthWest => CursorIcon::ResizeSouthWest,
        cssengine::CursorIcon::ResizeWest => CursorIcon::ResizeWest,
        cssengine::CursorIcon::ResizeNorthWest => CursorIcon::ResizeNorthWest,
        cssengine::CursorIcon::ResizeNorth => CursorIcon::ResizeNorth,
        cssengine::CursorIcon::ResizeNorthEast => CursorIcon::ResizeNorthEast,
        cssengine::CursorIcon::ResizeColumn => CursorIcon::ResizeColumn,
        cssengine::CursorIcon::ResizeRow => CursorIcon::ResizeRow,
        cssengine::CursorIcon::ZoomIn => CursorIcon::ZoomIn,
        cssengine::CursorIcon::ZoomOut => CursorIcon::ZoomOut,
    };

    s.interact_cursor.replace(cursor);
}
