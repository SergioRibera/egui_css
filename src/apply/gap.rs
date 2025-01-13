use cssengine::PxPct;
use egui::Ui;

pub(super) fn apply_gap(ui: &mut Ui, (row, col): (Option<PxPct>, Option<PxPct>)) {
    let available = ui.available_size();
    let spacing = ui.spacing_mut();

    if let Some(PxPct::Px(row)) = row {
        spacing.icon_spacing = row;
        spacing.item_spacing.x = row;
    }
    if let Some(PxPct::Pct(row)) = row {
        spacing.icon_spacing = row * available.x;
        spacing.item_spacing.x = row * available.x;
    }

    if let Some(PxPct::Px(col)) = col {
        spacing.item_spacing.y = col;
        spacing.menu_spacing = col;
    }
    if let Some(PxPct::Pct(col)) = col {
        spacing.item_spacing.y = col * available.y;
        spacing.menu_spacing = col * available.y;
    }
}
