use crate::color_item;
use eframe::egui::{self, RichText};
use std::sync::mpsc;
use std::thread;

const MARGIN: f32 = 10f32;

pub struct Circle {
    positions: Vec<f32>,
    width: u32,
    height: u32,
    // manage thread
    hthread: Option<thread::JoinHandle<()>>,
    channel: Option<mpsc::Receiver<image::DynamicImage>>,
    // manage drag
}

#[derive(Clone)]
struct CircleGenerator {
    data: Vec<(f32, color_item::ColorItem)>,
    width: u32,
    height: u32,
}

impl CircleGenerator {
    fn new(
        colors: Vec<color_item::ColorItem>,
        positions: Vec<f32>,
        width: u32,
        height: u32,
    ) -> Self {
        let mut data = Vec::new();
        for i in 0..colors.len() {
            data.push((positions[i], colors[i].clone()));
        }
        data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Self {
            data,
            width,
            height,
        }
    }
    fn get_color(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let dist = self.get_dist(x, y);
        let maxdist = self.get_dist_max();
        let dist_divided = dist / maxdist;
        for i in 1..self.data.len() {
            if dist_divided <= self.data[i].0 {
                let color2 = self.data[i - 1].1.clone();
                let color1 = self.data[i].1.clone();
                let color1_divided =
                    (dist_divided - self.data[i - 1].0) / (self.data[i].0 - self.data[i - 1].0);
                let color2_divided = 1.0 - color1_divided;
                return (
                    (color1.r as f32 * color1_divided + color2.r as f32 * color2_divided) as u8,
                    (color1.g as f32 * color1_divided + color2.g as f32 * color2_divided) as u8,
                    (color1.b as f32 * color1_divided + color2.b as f32 * color2_divided) as u8,
                );
            } else {
                continue;
            }
        }
        return (0, 0, 0);
    }
    fn get_dist(&self, x: u32, y: u32) -> f32 {
        let x = x as f32 - self.width as f32 / 2f32;
        let y = y as f32 - self.height as f32 / 2f32;
        return (x * x + y * y).sqrt();
    }
    fn get_dist_max(&self) -> f32 {
        return ((self.width * self.width + self.height * self.height) as f32).sqrt() / 2f32;
    }
}

impl Circle {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            width: 512,
            height: 512,
            hthread: None,
            channel: None,
        }
    }
    fn display_color(ui: &mut egui::Ui, position: &mut f32, color: &color_item::ColorItem) {
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
            ui.add(egui::Slider::new(position, 0.0..=1.0).fixed_decimals(2)
            );
            ui.label(
                RichText::new(crate::utils::resized_str(&color.name, 12))
                    .color(color.get_full_value_color32()),
            );
        });
    }

}

impl super::Generate for Circle {
    fn get_name(&self) -> String {
        return "\u{eae6} Circle".into();
    }
    fn paint_ui(
        &mut self,
        ui: &mut egui::Ui,
        colors: &Vec<color_item::ColorItem>,
    ) -> Option<image::DynamicImage> {
        if colors.len() < 2 {
            ui.label("Need at least 2 colors.");
            return None;
        }
        if self.positions.len() != colors.len() {
            self.positions.clear();
            for i in 0..colors.len() {
                let pos = i as f32 / (colors.len() - 1) as f32;
                self.positions.push(pos);
            }
        }
        let positions_len = self.positions.len();
        self.positions[0] = 0.0;
        self.positions[positions_len - 1] = 1.0;
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
        ui.horizontal(|ui| {
            if ui.button("\u{f0674} Generate").clicked() {
                let thread_colors = colors.clone();
                let thread_positions = self.positions.clone();
                let thread_width = self.width.clone();
                let thread_height = self.height.clone();
                let (tx, rx) = mpsc::channel();
                self.channel = Some(rx);
                self.hthread = Some(thread::spawn(move || {
                    // many colors
                    // sort first
                    let gen = CircleGenerator::new(
                        thread_colors,
                        thread_positions,
                        thread_width,
                        thread_height,
                    );

                    let buffer = image::RgbImage::from_fn(thread_width, thread_height, |x, y| {
                        let (r, g, b) = gen.get_color(x, y);
                        image::Rgb([r, g, b])
                    });
                    tx.send(image::DynamicImage::ImageRgb8(buffer)).unwrap();
                }));
            }
            if self.hthread.is_some() {
                ui.spinner();
            }
        });
        let width = 192f32;
        let highlight = egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (rect, response) =
                ui.allocate_exact_size(egui::Vec2::splat(width), egui::Sense::hover());
            let center_x = rect.left() + rect.width() / 2f32;
            let center_y = rect.top() + rect.height() / 2f32;
            let sqrt2_side = rect.width() / 2f32.sqrt();
            let painter = ui.painter();
            // detect selected
            let mut highlight = None;
            if let Some(pos) = response.hover_pos() {
                for i in 0..positions_len {
                    let pos2center = ((pos.x - center_x)*(pos.x-center_x) + (pos.y-center_y)*(pos.y-center_y)).sqrt();
                    if (pos2center - width*self.positions[i] / 2f32).abs() < 4f32 {
                        highlight = Some(i);
                        break;
                    }
                }
            }
            // show preview
            let center_pos = egui::pos2(center_x, center_y);
            let rect_pos = egui::Rect::from_center_size(center_pos, egui::Vec2::splat(sqrt2_side));
            let rect_stroke = egui::Stroke::new(2f32, egui::Color32::LIGHT_GRAY);
            painter.rect_stroke(rect_pos, 0.0, rect_stroke);
            for i in 0..colors.len() {
                let color = &colors[i];
                let pos = self.positions[i];
                let stroke = if Some(i) == highlight {
                    egui::Stroke::new(6f32, color.get_full_value_color32())
                } else {
                    egui::Stroke::new(2f32, color.to_color32())
                };
                let radius = width * pos / 2f32;
                painter.circle_stroke(center_pos, radius, stroke);
            }
            return highlight;
        });
        if let Some(hl) = highlight.inner {
            ui.label(RichText::new(crate::utils::resized_str(&colors[hl].name, 24)).color(colors[hl].get_full_value_color32()));
        }
        else {
            ui.label("None");
        }

        ui.separator();
        let mut cursor = ui.cursor();
        let size = ui.available_size();
        cursor.set_height(size.y - MARGIN);
        cursor.set_width(size.x);
        ui.allocate_ui_at_rect(cursor, |ui| {
            egui::ScrollArea::new([false, true])
                .scroll_bar_visibility(
                    egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible,
                )
                .min_scrolled_height(size.y - MARGIN)
                .id_source("line_list")
                .show(ui, |ui| {
                    let mut size = ui.available_size();
                    size.y = 10f32;
                    ui.add_sized(size, egui::Label::new("\u{f0835} Positions"));
                    for i in 0..colors.len() {
                        Self::display_color(ui, &mut self.positions[i], &colors[i]);
                    }
                });
        });
        if let Some(hth) = &self.hthread {
            if hth.is_finished() {
                if let Some(rx) = self.channel.take() {
                    return rx.recv().ok();
                }
                self.hthread = None;
                self.channel = None;
            }
        }
        return None;
    }
}