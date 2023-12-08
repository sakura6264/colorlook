use crate::color_item;
use eframe::egui;
use egui::RichText;

pub struct Mono {
    width: u32,
    height: u32,
}

impl Mono {
    pub fn new() -> Self {
        Self {
            width: 512,
            height: 512,
        }
    }
    fn display_color(ui: &mut egui::Ui, color: &color_item::ColorItem) -> bool {
        let mut ret = false;
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(5f32, ui.text_style_height(&egui::TextStyle::Body)),
                egui::Sense {
                    click: false,
                    drag: false,
                    focusable: false,
                },
            );
            let painter = ui.painter();
            painter.rect(
                rect,
                0f32,
                color.to_color32(),
                egui::Stroke::new(0.5f32, egui::Color32::WHITE),
            );
            ret = ui.button("\u{eb2a} Paint").clicked();
            ui.label(
                RichText::new(crate::utils::resized_str(&color.name, 12))
                    .color(color.get_full_value_color32()),
            );
        });
        return ret;
    }
}

impl super::Generate for Mono {
    fn get_name(&self) -> String {
        return "\u{eae6} Mono".into();
    }
    fn paint_ui(
        &mut self,
        ui: &mut egui::Ui,
        colors: &Vec<color_item::ColorItem>,
    ) -> Option<image::DynamicImage> {
        ui.horizontal(|ui| {
            ui.label("\u{f019e} Width:");
            ui.add(
                egui::DragValue::new(&mut self.width)
                    .speed(1.0)
                    .clamp_range(1..=16384),
            );
            ui.label("\u{f019e} Height:");
            ui.add(
                egui::DragValue::new(&mut self.height)
                    .speed(1.0)
                    .clamp_range(1..=16384),
            );
        });
        ui.separator();

        let mut wait4gen = None;
        for i in 0..colors.len() {
            if Self::display_color(ui, &colors[i]) {
                wait4gen = Some(colors[i].clone());
            }
        }

        match wait4gen {
            Some(color) => {
                let buffer = image::RgbImage::from_pixel(
                    self.width,
                    self.height,
                    image::Rgb([color.r, color.g, color.b]),
                );
                let img = image::DynamicImage::ImageRgb8(buffer);
                return Some(img);
            }
            None => return None,
        }
    }
}
