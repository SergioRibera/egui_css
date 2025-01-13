use cssengine::{BoxShadow, Color, Declaration, PxPct, PxPctAuto, TextOverflow};
use egui::{Color32, Rounding, Shadow, TextWrapMode, Ui, Vec2};

mod cursor;
mod distances;
mod gap;

use cursor::*;
use distances::*;
use gap::*;

#[derive(Clone, Copy)]
pub(super) enum Orientation {
    Both,
    Horizontal,
    Vertical,
    Top,
    Bottom,
    Right,
    Left,
}

pub(super) fn pxpct_auto(available: Vec2, orientation: Orientation, v: PxPctAuto) -> f32 {
    match v {
        PxPctAuto::Px(v) => v,
        PxPctAuto::Auto => available.length(),
        PxPctAuto::Pct(v) => {
            let mul = match orientation {
                Orientation::Horizontal | Orientation::Left | Orientation::Right => available.x,
                Orientation::Vertical | Orientation::Top | Orientation::Bottom => available.y,
                _ => available.length(),
            };

            v * mul
        }
    }
}

pub(self) fn pxpct(available: Vec2, orientation: Orientation, v: PxPct) -> f32 {
    match v {
        PxPct::Px(v) => v,
        PxPct::Pct(v) => {
            let mul = match orientation {
                Orientation::Horizontal | Orientation::Left | Orientation::Right => available.x,
                Orientation::Vertical | Orientation::Top | Orientation::Bottom => available.y,
                _ => available.length(),
            };

            v * mul
        }
    }
}

pub(super) fn text_overflow(v: TextOverflow) -> Option<TextWrapMode> {
    match v {
        TextOverflow::Wrap => Some(TextWrapMode::Wrap),
        TextOverflow::Ellipsis => Some(TextWrapMode::Truncate),
        TextOverflow::Clip => None,
    }
}

pub(super) fn into_color(c: Color) -> Color32 {
    let [r, g, b, a] = c.to_rgba8();
    Color32::from_rgba_unmultiplied(r, g, b, a)
}

pub(super) fn into_box(available: Vec2, b: BoxShadow) -> Shadow {
    let ox = pxpct(available, Orientation::Horizontal, b.h_offset);
    let oy = pxpct(available, Orientation::Vertical, b.v_offset);
    let blur = pxpct(available, Orientation::Both, b.blur_radius);
    let spread = pxpct(available, Orientation::Both, b.spread);

    Shadow {
        offset: Vec2::new(ox, oy),
        blur,
        spread,
        color: into_color(b.color),
    }
}

#[inline(never)]
pub fn apply_style(decl: Declaration, ui: &mut Ui) {
    let available = ui.available_size();
    let style = ui.style_mut();

    match decl {
        Declaration::None => ui.reset_style(),
        Declaration::Width(v) => {
            if v != PxPctAuto::Auto {
                ui.set_width(pxpct_auto(available, Orientation::Horizontal, v));
            }
        }
        Declaration::Height(v) => {
            if v != PxPctAuto::Auto {
                ui.set_height(pxpct_auto(available, Orientation::Vertical, v));
            }
        }
        Declaration::MinWidth(v) => {
            if v != PxPctAuto::Auto {
                ui.set_min_width(pxpct_auto(available, Orientation::Horizontal, v));
            }
        }
        Declaration::MinHeight(v) => {
            if v != PxPctAuto::Auto {
                ui.set_min_height(pxpct_auto(available, Orientation::Vertical, v));
            }
        }
        Declaration::MaxWidth(v) => {
            if v != PxPctAuto::Auto {
                ui.set_max_width(pxpct_auto(available, Orientation::Horizontal, v));
            }
        }
        Declaration::MaxHeight(v) => {
            if v != PxPctAuto::Auto {
                ui.set_max_height(pxpct_auto(available, Orientation::Vertical, v));
            }
        }
        Declaration::BorderColor(c) => {
            let visuals = ui.visuals_mut();
            let c = into_color(c);
            visuals.window_stroke.color = c;
            visuals.widgets.active.bg_stroke.color = c;
        }
        Declaration::Border(b) => {
            if let Some(color) = b.color {
                let visuals = ui.visuals_mut();
                let c = into_color(color);
                visuals.window_stroke.color = c;
                visuals.widgets.active.bg_stroke.color = c;
            }
            if let Some(w) = b.width {
                let visuals = ui.visuals_mut();
                let w = pxpct(available, Orientation::Both, w);
                visuals.window_stroke.width = w;
                visuals.widgets.active.bg_stroke.width = w;
                ui.visuals_mut().window_stroke.width = w;
            }
        }
        Declaration::BorderWidth(v) => ui.visuals_mut().window_stroke.width = v.0,
        Declaration::BorderRadius(v) => {
            let visuals = ui.visuals_mut();
            let value = Rounding::same(pxpct(available, Orientation::Both, v));
            visuals.window_rounding = value;
            visuals.menu_rounding = value;
        }

        Declaration::Padding(v) => apply_padding(style, available, Orientation::Both, v),
        Declaration::PaddingLeft(v) => apply_padding(style, available, Orientation::Left, v),
        Declaration::PaddingTop(v) => apply_padding(style, available, Orientation::Top, v),
        Declaration::PaddingRight(v) => apply_padding(style, available, Orientation::Right, v),
        Declaration::PaddingBottom(v) => apply_padding(style, available, Orientation::Bottom, v),

        Declaration::Margin(v) => {
            let v = apply_margin(style, available, Orientation::Both, v);
            ui.add_space(v);
        }
        Declaration::MarginLeft(v) => {
            let v = apply_margin(style, available, Orientation::Left, v);
            ui.add_space(v);
        }
        Declaration::MarginTop(v) => {
            let v = apply_margin(style, available, Orientation::Top, v);
            ui.add_space(v);
        }
        Declaration::MarginRight(v) => {
            let v = apply_margin(style, available, Orientation::Right, v);
            ui.add_space(v);
        }
        Declaration::MarginBottom(v) => {
            let v = apply_margin(style, available, Orientation::Bottom, v);
            ui.add_space(v);
        }

        Declaration::Cursor(v) => apply_cursor(ui.visuals_mut(), v),
        Declaration::CursorColor(v) => ui.visuals_mut().text_cursor.stroke.color = into_color(v),
        Declaration::Color(v) => {
            ui.visuals_mut().override_text_color.replace(into_color(v));
        }
        Declaration::BackgroundColor(v) => {
            let v = into_color(v);
            let visuals = ui.visuals_mut();
            visuals.panel_fill = v;
            visuals.window_fill = v;
            visuals.faint_bg_color = v;
            visuals.widgets.active.bg_fill = v;
        }
        Declaration::BoxShadow(b) => {
            let b = into_box(available, b);
            let visuals = ui.visuals_mut();
            visuals.popup_shadow = b;
            visuals.window_shadow = b;
        }
        Declaration::ColGap(v) => apply_gap(ui, (None, Some(v))),
        Declaration::RowGap(v) => apply_gap(ui, (Some(v), None)),
        Declaration::Gap(v) => apply_gap(ui, (Some(v.0), v.1)),
        Declaration::TextOverflow(v) => style.wrap_mode = text_overflow(v),

        // TODO: custom widget to handle animations
        Declaration::Transition((_, t)) => {
            style.animation_time = t.duration as f32;
        }

        // TODO: handle taffy
        Declaration::Display(_) => {}
        Declaration::Position(_) => {}

        Declaration::FlexDirection(_) => {}
        Declaration::FlexWrap(_) => {}
        Declaration::FlexGrow(_) => {}
        Declaration::FlexShrink(_) => {}
        Declaration::FlexBasis(_) => {}

        Declaration::JustifyContent(_) => {}
        Declaration::JustifySelf(_) => {}

        Declaration::AlignSelf(_) => {}
        Declaration::AlignItems(_) => {}
        Declaration::AlignContent(_) => {}
        // IDK how handle that
        Declaration::UserSelect(_) => {}
        Declaration::ZIndex(_) => {}
        Declaration::BorderLeft(_) => {}
        Declaration::BorderTop(_) => {}
        Declaration::BorderRight(_) => {}
        Declaration::BorderBottom(_) => {}
        Declaration::AspectRatio(_) => {}
        Declaration::InsetLeft(_) => {}
        Declaration::InsetTop(_) => {}
        Declaration::InsetRight(_) => {}
        Declaration::InsetBottom(_) => {}
        Declaration::OutlineColor(_) => {}
        Declaration::Outline(_) => {}

        Declaration::FontSize(_) => {}
        Declaration::FontFamily(_) => {}
        Declaration::FontWeight(_) => {}
        Declaration::FontStyle(_) => {}
        Declaration::LineHeight(_) => {}
    }
}
