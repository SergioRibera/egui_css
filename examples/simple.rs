use egui::{Button, Label};
use egui_css::{StyleSheet, StyledWidgetExt};

fn main() -> eframe::Result {
    eframe::run_native(
        "egui_css simple",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MainApp::new(cc)))),
    )
}

#[derive(Default)]
pub struct MainApp {
    age: u32,
    css: String,
}

impl MainApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let css = r#".header {
    color: purple;
    padding: 20px;
    background-color: black;
    height: 50px;
}

#counter {
    color: green;
    background-color: white;
    padding: 20px;
    margin: 20px;
}

#counter:hover {
    color: purple;
}

.header:hover {
    color: blue;
}"#;
        egui_css::change_style(StyleSheet::from_css(&css));

        Self {
            css: css.to_string(),
            ..Default::default()
        }
    }
}

impl eframe::App for MainApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Sides::new()
                .height(ui.available_height())
                .spacing(20.0)
                .show(
                    ui,
                    |ui: &mut egui::Ui| {
                        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                            let mut layout_job = egui_extras::syntax_highlighting::highlight(
                                ui.ctx(),
                                ui.style(),
                                &egui_extras::syntax_highlighting::CodeTheme::from_style(
                                    ui.style().as_ref(),
                                ),
                                string,
                                "css",
                            );
                            layout_job.wrap.max_width = wrap_width;
                            ui.fonts(|f| f.layout_job(layout_job))
                        };
                        ui.vertical(|ui| {
                            if ui
                                .add(
                                    egui::TextEdit::multiline(&mut self.css)
                                        .font(egui::TextStyle::Monospace) // for cursor height
                                        .code_editor()
                                        // .desired_rows(10)
                                        .lock_focus(true)
                                        .layouter(&mut layouter),
                                )
                                .changed()
                            {
                                egui_css::change_style(StyleSheet::from_css(&self.css));
                            }
                        });
                    },
                    |ui| {
                        ui.vertical(|ui| {
                            ui.add(Label::new("My egui Application").styled().class(".header"));
                            if ui
                                .add(Button::new("Increment").styled().css_id("counter"))
                                .clicked()
                            {
                                self.age += 1;
                            }
                            ui.add(
                                Label::new(format!("age {}", self.age))
                                    .styled()
                                    .class(".text-orange-700"),
                            );
                            ui.add(Label::new("No styled"));
                        })
                    },
                );
        });
    }
}
