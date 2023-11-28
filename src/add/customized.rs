use crate::color_item;
use eframe::egui::{self, RichText};

pub struct Customized {
    name: String,
    color: Color,
}

#[derive(Clone)]
enum Color {
    RGB((u8, u8, u8)),
    HEX(String),
    HSV((f32, f32, f32)),
}

impl Customized {
    pub fn new() -> Self {
        Self {
            name: super::get_random_name(8),
            color: Color::RGB((0, 0, 0)),
        }
    }
}

impl super::AddColor for Customized {
    fn get_name(&self) -> String {
        return "\u{eae6} Customized Color".into();
    }
    fn paint_ui(&mut self, ui: &mut egui::Ui) -> Option<Vec<crate::color_item::ColorItem>> {
        let mut ret = None;
        ui.horizontal(|ui| {
            ui.label("\u{f1050} Name:");
            ui.text_edit_singleline(&mut self.name);
        });
        let mut color_change = None;
        let color;
        match self.color {
            Color::RGB((ref mut r, ref mut g, ref mut b)) => {
                color = color_item::ColorItem {
                    name: self.name.clone(),
                    r: *r,
                    g: *g,
                    b: *b,
                };
                ui.horizontal(|ui| {
                    if ui.button("\u{ea60} Add").clicked() {
                        ret = Some(vec![color.clone()]);
                        self.name = super::get_random_name(8);
                    }
                    if ui.button("\u{f0ae4} RGB").clicked() {
                        color_change = Some(Color::HEX(color.get_hex()));
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("R:").color(egui::Color32::RED));
                    ui.add(egui::DragValue::new(r).clamp_range(0..=255).speed(1.0));
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("G:").color(egui::Color32::GREEN));
                    ui.add(egui::DragValue::new(g).clamp_range(0..=255).speed(1.0));
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("B:").color(egui::Color32::BLUE));
                    ui.add(egui::DragValue::new(b).clamp_range(0..=255).speed(1.0));
                });
            }
            Color::HEX(ref mut hex) => {
                let c = color_item::ColorItem::from_hex(hex, self.name.as_str());
                color = match &c {
                    Some(e) => e.clone(),
                    None => color_item::ColorItem {
                        name: self.name.clone(),
                        r: 0,
                        g: 0,
                        b: 0,
                    },
                };
                ui.horizontal(|ui| {
                    if ui.button("\u{ea60} Add").clicked() {
                        ret = Some(vec![color.clone()]);
                        self.name = super::get_random_name(8);
                    }
                    if ui.button("\u{f12a7} HEX").clicked() {
                        color_change = Some(Color::HSV((color.get_h(), color.get_s(), color.get_v())));
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("HEX:");
                    ui.text_edit_singleline(hex);
                });
                if c.is_none() {
                    ui.label("Warning: Invalid HEX");
                }
            },
            Color::HSV((ref mut h, ref mut s, ref mut v)) => {
                color = color_item::ColorItem::from_hsv(*h, *s, *v, self.name.as_str());
                ui.horizontal(|ui| {
                    if ui.button("\u{ea60} Add").clicked() {
                        ret = Some(vec![color.clone()]);
                        self.name = super::get_random_name(8);
                    }
                    if ui.button("\u{f04c5} HSV").clicked() {
                        color_change = Some(Color::RGB((color.r, color.g, color.b)));
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("H:").color(egui::Color32::LIGHT_BLUE));
                    ui.add(egui::DragValue::new(h).clamp_range(0f32..=360f32).fixed_decimals(2).speed(1.0));
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("S:").color(egui::Color32::KHAKI));
                    ui.add(egui::DragValue::new(s).clamp_range(0f32..=1f32).fixed_decimals(2).speed(0.01));
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("V:").color(egui::Color32::WHITE));
                    ui.add(egui::DragValue::new(v).clamp_range(0f32..=1f32).fixed_decimals(2).speed(0.01));
                });
            }
        }
        // preview it use painter
        ui.label("\u{eb28} Preview:");
        let painter = ui.painter();
        let mut cursor = ui.cursor();
        cursor.set_height(60f32);

        painter.rect(
            cursor,
            0f32,
            color.to_color32(),
            egui::Stroke::new(2f32, egui::Color32::WHITE),
        );

        if let Some(color_change) = color_change {
            self.color = color_change;
        }

        return ret;
    }
}
