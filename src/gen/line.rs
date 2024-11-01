use crate::color_item;
use eframe::egui::{self, RichText};
use std::sync::mpsc;
use std::thread;


pub struct Line {
    positions: Vec<f32>,
    angel: f32,
    width: u32,
    height: u32,
    // manage thread
    hthread: Option<thread::JoinHandle<()>>,
    channel: Option<mpsc::Receiver<image::DynamicImage>>,
    // manage drag
}

#[derive(Clone)]
struct LineGenerator {
    data: Vec<(f32, color_item::ColorItem)>,
    angel: f32,
    linemax: f32,
}

impl LineGenerator {
    fn new(
        colors: Vec<color_item::ColorItem>,
        positions: Vec<f32>,
        angel: f32,
        width: u32,
        height: u32,
    ) -> Self {
        let mut data = Vec::new();
        for i in 0..colors.len() {
            data.push((positions[i], colors[i].clone()));
        }
        data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let linemax = (width as f32) * angel.sin() + (height as f32) * angel.cos();
        Self {
            data,
            angel,
            linemax,
        }
    }
    fn get_color(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let line = (x as f32) * self.angel.sin() + (y as f32) * self.angel.cos();
        let line_divided = line / self.linemax;
        for i in 1..self.data.len() {
            if line_divided <= self.data[i].0 {
                let color2 = self.data[i - 1].1.clone();
                let color1 = self.data[i].1.clone();
                let color1_divided =
                    (line_divided - self.data[i - 1].0) / (self.data[i].0 - self.data[i - 1].0);
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
}

impl Line {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            angel: 0.0,
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
            ui.add(egui::Slider::new(position, 0.0..=1.0).fixed_decimals(2));
            ui.label(
                RichText::new(crate::utils::resized_str(&color.name, 12))
                    .color(color.get_full_value_color32()),
            );
        });
    }
}

impl super::Generate for Line {
    fn get_name(&self) -> String {
        return "\u{eae6} Line".into();
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
            ui.label("\u{f0937} Angle:");
            ui.add(egui::Slider::new(&mut self.angel, 0.0..=90.0).text("°"));
        });
        ui.horizontal(|ui| {
            ui.label("\u{f019e} Width:");
            ui.add(
                egui::DragValue::new(&mut self.width)
                    .speed(1.0)
                    .range(1..=16384),
            );
            ui.label("\u{f019e} Height:");
            ui.add(
                egui::DragValue::new(&mut self.height)
                    .speed(1.0)
                    .range(1..=16384),
            );
        });
        ui.horizontal(|ui| {
            if ui.button("\u{f0674} Generate").clicked() {
                let thread_colors = colors.clone();
                let thread_positions = self.positions.clone();
                let thread_angel = self.angel.to_radians();
                let thread_width = self.width.clone();
                let thread_height = self.height.clone();
                let (tx, rx) = mpsc::channel();
                self.channel = Some(rx);
                self.hthread = Some(thread::spawn(move || {
                    // many colors
                    // sort first
                    let gen = LineGenerator::new(
                        thread_colors,
                        thread_positions,
                        thread_angel,
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
        let height = 20f32;
        let highlight = egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (rect, response) =
                ui.allocate_exact_size(egui::Vec2::new(width, height), egui::Sense::hover());
            let painter = ui.painter();
            // detect selected
            let mut highlight = None;
            if let Some(pos) = response.hover_pos() {
                for i in 0..positions_len {
                    if (pos.x - rect.left() - rect.width() * self.positions[i]).abs() < 4f32 {
                        highlight = Some(i);
                        break;
                    }
                }
            }
            for i in 0..colors.len() {
                let color = &colors[i];
                let pos = self.positions[i];
                let stroke = if Some(i) == highlight {
                    egui::Stroke::new(6f32, color.get_full_value_color32())
                } else {
                    egui::Stroke::new(2f32, color.to_color32())
                };
                let pos_x = rect.left() + pos * rect.width();
                painter.vline(
                    pos_x,
                    eframe::emath::Rangef::new(rect.top(), rect.bottom()),
                    stroke,
                );
            }
            return highlight;
        });
        if let Some(hl) = highlight.inner {
            ui.label(
                RichText::new(crate::utils::resized_str(&colors[hl].name, 24))
                    .color(colors[hl].get_full_value_color32()),
            );
        } else {
            ui.label("None");
        }

        ui.separator();

        ui.label("\u{f0835} Positions:");
        for i in 0..colors.len() {
            Self::display_color(ui, &mut self.positions[i], &colors[i]);
        }

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
