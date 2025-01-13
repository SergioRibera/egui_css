use std::ops::{Deref, DerefMut};

use cssengine::{Declaration, PseudoClass};
use egui::{Response, Ui, Widget};
use smallvec::SmallVec;

use crate::apply::{apply_style, pxpct_auto, Orientation};

pub struct StyledWidget<W: Widget> {
    class: Option<String>,
    id: Option<String>,
    widget: W,
}

impl<W: Widget> StyledWidget<W> {
    pub fn new(widget: W) -> Self {
        Self {
            widget,
            class: None,
            id: None,
        }
    }

    pub fn css_id(mut self, id: impl ToString) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn class(mut self, class: impl ToString) -> Self {
        self.class = Some(class.to_string());
        self
    }
}

impl<W: Widget> ToString for StyledWidget<W> {
    fn to_string(&self) -> String {
        let mut result = String::new();

        if let Some(ref id) = self.id {
            result.push('#');
            result.push_str(&id.to_string());
        }

        if let Some(ref class) = self.class {
            if !result.is_empty() {
                result.push(' ');
            }
            for class_name in class.split_whitespace() {
                if !class_name.starts_with('.') {
                    result.push('.');
                }
                result.push_str(class_name);
            }
        }

        if result.is_empty() {
            format!("* {}", self.name())
        } else {
            result
        }
    }
}

impl<W: Widget> Widget for StyledWidget<W> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut end_space = None;

        // Helper function to handle margin styles and update `end_space`
        let mut handle_margin = |ui: &mut Ui, decl: &Declaration| {
            if let Declaration::Margin(v) = decl {
                let available = ui.available_size();
                end_space.replace(pxpct_auto(available, Orientation::Both, v.clone()));
            }
        };

        // Retrieve global styles and apply base styles
        let mut pseudo_classes = SmallVec::<[_; 32]>::new();
        if let Some((ps_class, style)) = crate::GLOBAL_STYLES.lock().as_mut().and_then(|style| {
            let styles = style.get_styles(self.to_string());
            if styles.is_empty() {
                None
            } else {
                Some(styles.into_iter().partition(|(p, _)| p.is_some()))
            }
        }) {
            pseudo_classes = ps_class;
            style.iter().for_each(|(_, decls)| {
                decls.iter().for_each(|decl| {
                    apply_style(decl.clone(), ui);
                    handle_margin(ui, decl);
                });
            });
        } else {
            println!("Not found query: {}", self.to_string());
        }

        let res = self.widget.ui(ui);

        // Process pseudo classes (:hover, :active, etc.)
        pseudo_classes.iter().for_each(|(ps_class, decls)| {
            let Some(ps_class) = ps_class else {
                return;
            };
            let condition_met = match ps_class {
                PseudoClass::Hover => res.hovered(),
                PseudoClass::Active => res.enabled(),
                PseudoClass::ActiveHover => res.enabled() && res.hovered(),
                PseudoClass::Disabled => !res.enabled(),
                PseudoClass::DisabledHover => !res.enabled() && res.hovered(),
                PseudoClass::Focus => res.has_focus(),
                PseudoClass::FocusHover => res.has_focus() && res.hovered(),
                // IDK how handle this
                PseudoClass::Placeholder => false,
                PseudoClass::Selection => false,
            };

            if condition_met {
                decls.iter().for_each(|decl| {
                    apply_style(decl.clone(), ui);
                    handle_margin(ui, decl);
                });
            }
        });

        if let Some(space) = end_space {
            ui.add_space(space);
        }

        // Reset styles for the next widget
        ui.reset_style();

        res
    }
}

pub trait StyledWidgetExt: Widget + Sized {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    fn styled(self) -> StyledWidget<Self> {
        StyledWidget::new(self)
    }
}

impl<W: Widget> Deref for StyledWidget<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl<W: Widget> DerefMut for StyledWidget<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}

impl<W: Widget> StyledWidgetExt for W {}
