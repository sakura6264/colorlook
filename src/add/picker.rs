use crate::color_item;
use eframe::egui;

pub struct Picker {
    name: String,
    color: egui::Color32,
}

impl Picker {
    pub fn new() -> Self {
        Self {
            name: super::get_random_name(8),
            color: egui::Color32::BLACK,
        }
    }
}

impl super::AddColor for Picker {
    fn get_name(&self) -> String {
        return "\u{eae6} Color Picker".into();
    }
    fn paint_ui(&mut self, ui: &mut egui::Ui) -> Option<Vec<color_item::ColorItem>> {
        let mut ret = None;
        ui.horizontal(|ui| {
            ui.label("\u{f1050} Name:");
            ui.text_edit_singleline(&mut self.name);
        });
        if ui.button("\u{ea60} Add").clicked() {
            ret = Some(vec![color_item::ColorItem {
                name: self.name.clone(),
                r: self.color.r(),
                g: self.color.g(),
                b: self.color.b(),
            }]);
            self.name = super::get_random_name(8);
        }
        egui::color_picker::color_picker_color32(
            ui,
            &mut self.color,
            egui::color_picker::Alpha::Opaque,
        );
        return ret;
    }
}
