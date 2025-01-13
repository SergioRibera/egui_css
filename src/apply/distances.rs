use cssengine::{PxPct, PxPctAuto};
use egui::{Margin, Style, Vec2};

use super::{pxpct, pxpct_auto, Orientation};

pub(super) fn apply_margin(
    style: &mut Style,
    available: Vec2,
    orientation: Orientation,
    margin: PxPctAuto,
) -> f32 {
    let v = pxpct_auto(available, orientation, margin);
    match orientation {
        Orientation::Horizontal => {
            style.spacing.window_margin.left = v;
            style.spacing.menu_margin.right = v;

            style.spacing.window_margin.left = v;
            style.spacing.menu_margin.right = v;
        }
        Orientation::Vertical => {
            style.spacing.window_margin.top = v;
            style.spacing.menu_margin.bottom = v;

            style.spacing.window_margin.top = v;
            style.spacing.menu_margin.bottom = v;
        }
        Orientation::Left => {
            style.spacing.window_margin.left = v;
            style.spacing.menu_margin.left = v;
        }
        Orientation::Right => {
            style.spacing.window_margin.right = v;
            style.spacing.menu_margin.right = v;
        }
        Orientation::Top => {
            style.spacing.window_margin.top = v;
            style.spacing.menu_margin.top = v;
        }
        Orientation::Bottom => {
            style.spacing.window_margin.bottom = v;
            style.spacing.menu_margin.bottom = v;
        }
        _ => {
            let margin = Margin::symmetric(v, v);
            style.spacing.window_margin = margin;
            style.spacing.menu_margin = margin;
        }
    }
    v
}

pub(super) fn apply_padding(
    style: &mut Style,
    available: Vec2,
    orientation: Orientation,
    padding: PxPct,
) {
    let v = pxpct(available, orientation, padding);
    match orientation {
        Orientation::Horizontal | Orientation::Left | Orientation::Right => {
            style.spacing.button_padding.x = v
        }
        Orientation::Vertical | Orientation::Top | Orientation::Bottom => {
            style.spacing.button_padding.y = v
        }
        _ => {
            style.spacing.button_padding.x = v;
            style.spacing.button_padding.y = v;
        }
    }
}
