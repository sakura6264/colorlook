use crate::color_item;
use eframe::egui;

pub struct Blocks {
    width: u32,
    height: u32,
    x_num: u32,
    y_num: u32,
}

impl Blocks {
    pub fn new() -> Self {
        Self {
            width: 512,
            height: 512,
            x_num: 8,
            y_num: 8,
        }
    }
}

#[derive(Clone)]
struct BlocksGenerator {
    data: Vec<Vec<(u8, u8, u8)>>,
    width: u32,
    height: u32,
    x_num: u32,
    y_num: u32,
}

impl BlocksGenerator {
    fn new(
        colors: Vec<color_item::ColorItem>,
        width: u32,
        height: u32,
        x_num: u32,
        y_num: u32,
    ) -> Self {
        let mut data = Vec::new();
        for i in 0..y_num {
            let mut row = Vec::new();
            for j in 0..x_num {
                let index = (i * x_num + j) as usize;
                row.push(match colors.get(index) {
                    Some(color) => (color.r, color.g, color.b),
                    None => (0, 0, 0),
                });
            }
            data.push(row);
        }
        Self {
            data,
            width,
            height,
            x_num,
            y_num,
        }
    }
    fn get_color(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let x_index = x * self.x_num / self.width;
        let y_index = y * self.y_num / self.height;
        self.data[y_index as usize][x_index as usize]
    }
}

impl super::Generate for Blocks {
    fn get_name(&self) -> String {
        "\u{eae6} Blocks".into()
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
            ui.label("\u{f01d8} X:");
            ui.add(
                egui::DragValue::new(&mut self.x_num)
                    .speed(1.0)
                    .range(1..=16384),
            );
            ui.label("\u{f01d9} Y:");
            ui.add(
                egui::DragValue::new(&mut self.y_num)
                    .speed(1.0)
                    .range(1..=16384),
            );
        });
        if ui.button("\u{f0674} Generate").clicked() {
            let gen = BlocksGenerator::new(
                colors.clone(),
                self.width,
                self.height,
                self.x_num,
                self.y_num,
            );
            let buffer = image::RgbImage::from_fn(self.width, self.height, |x, y| {
                let color = gen.get_color(x, y);
                image::Rgb([color.0, color.1, color.2])
            });
            return Some(image::DynamicImage::ImageRgb8(buffer));
        }
        None
    }
}
