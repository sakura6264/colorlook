use eframe::egui::{self, RichText};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub enum VecOp {
    MoveUp,
    MoveDown,
    MoveTop,
    MoveBottom,
    Duplicate,
    Delete,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ColorItem {
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorItem {
    pub fn get_hex(&self) -> String {
        return format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b);
    }

    pub fn from_hex(hex: &str, name: &str) -> Option<Self> {
        let mut hex = hex.to_string();
        if hex.starts_with("#") {
            hex.remove(0);
        }
        if hex.len() != 6 {
            return None;
        }
        let r = Self::str2u8(&hex[0..2])?;
        let g = Self::str2u8(&hex[2..4])?;
        let b = Self::str2u8(&hex[4..6])?;
        return Some(Self {
            name: name.to_string(),
            r,
            g,
            b,
        });
    }

    pub fn from_hsv(h: f32, s: f32, v: f32, name: &str) -> Self {
        let h_i = ((h % 360.0) / 60.0).floor() as i32;
        let f = (h % 360.0) / 60.0 - h_i as f32;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);
        let (r, g, b) = match h_i {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };
        Self {
            name: name.to_string(),
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        }
    }

    pub fn str2u8(s: &str) -> Option<u8> {
        let mut s = s.to_string();
        if s.len() != 2 {
            return None;
        }
        return Some(Self::char2u8(s.remove(0))? * 16 + Self::char2u8(s.remove(0))?);
    }

    pub fn char2u8(c: char) -> Option<u8> {
        match c {
            '0' => Some(0),
            '1' => Some(1),
            '2' => Some(2),
            '3' => Some(3),
            '4' => Some(4),
            '5' => Some(5),
            '6' => Some(6),
            '7' => Some(7),
            '8' => Some(8),
            '9' => Some(9),
            'a' | 'A' => Some(10),
            'b' | 'B' => Some(11),
            'c' | 'C' => Some(12),
            'd' | 'D' => Some(13),
            'e' | 'E' => Some(14),
            'f' | 'F' => Some(15),
            _ => None,
        }
    }
    pub fn get_h(&self) -> f32 {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        let mut h = 0.0;
        if max == min {
            h = 0.0;
        } else if max == self.r {
            h = 60.0 * (self.g as i16 - self.b as i16) as f32 / (max - min) as f32;
        } else if max == self.g {
            h = 60.0 * (self.b as i16 - self.r as i16) as f32 / (max - min) as f32 + 120.0;
        } else if max == self.b {
            h = 60.0 * (self.r as i16 - self.g as i16) as f32 / (max - min) as f32 + 240.0;
        }
        if h < 0.0 {
            h += 360.0;
        }
        return h;
    }
    pub fn get_s(&self) -> f32 {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);
        if max == 0 {
            return 0.0;
        }
        return (max - min) as f32 / max as f32;
    }
    pub fn get_v(&self) -> f32 {
        return self.r.max(self.g).max(self.b) as f32 / 255.0;
    }
    pub fn get_full_value_color32(&self) -> egui::Color32 {
        let max = self.r.max(self.g).max(self.b);
        let scale = 255.0 / max as f32;
        return egui::Color32::from_rgb(
            (self.r as f32 * scale) as u8,
            (self.g as f32 * scale) as u8,
            (self.b as f32 * scale) as u8,
        );
    }
    #[allow(unused)]
    pub fn to_color32(&self) -> egui::Color32 {
        return egui::Color32::from_rgb(self.r, self.g, self.b);
    }
}

pub fn draw_color_items(ui: &mut egui::Ui, colors: &mut Vec<ColorItem>) {
    let mut op = None;
    let mut index = 0;
    for i in 0..colors.len() {
        let color = &mut colors[i];
        let newcolor = ui.horizontal(|ui| {
            let mut rgb = [color.r, color.g, color.b];
            egui::color_picker::color_edit_button_srgb(ui, &mut rgb);
            ui.label(&color.name);
            ui.separator();
            ui.label(RichText::new(&color.get_hex()).color(color.get_full_value_color32()));
            return rgb;
        });
        color.r = newcolor.inner[0];
        color.g = newcolor.inner[1];
        color.b = newcolor.inner[2];
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("\u{eaa1}").on_hover_text("move up").clicked() {
                op = Some(VecOp::MoveUp);
                index = i;
            }
            if ui.button("\u{ea9a}").on_hover_text("move down").clicked() {
                op = Some(VecOp::MoveDown);
                index = i;
            }
            if ui.button("\u{eaf4}").on_hover_text("move top").clicked() {
                op = Some(VecOp::MoveTop);
                index = i;
            }
            if ui.button("\u{eaf3}").on_hover_text("move bottom").clicked() {
                op = Some(VecOp::MoveBottom);
                index = i;
            }
            if ui.button("\u{f0191}").on_hover_text("duplicate").clicked() {
                op = Some(VecOp::Duplicate);
                index = i;
            }
            if ui.button("\u{ea76}").on_hover_text("remove").clicked() {
                op = Some(VecOp::Delete);
                index = i;
            }
            if ui.button("\u{ebcc}").on_hover_text("copy hex").clicked() {
                let color = &colors[i];
                ui.output_mut(|o| {
                    o.copied_text = color.get_hex();
                });
            }
        });
    }
    if let Some(op) = op {
        match op {
            VecOp::MoveUp => {
                if index > 0 {
                    colors.swap(index, index - 1);
                }
            }
            VecOp::MoveDown => {
                if index < colors.len() - 1 {
                    colors.swap(index, index + 1);
                }
            }
            VecOp::MoveTop => {
                if index > 0 {
                    let color = colors.remove(index);
                    colors.insert(0, color);
                }
            }
            VecOp::MoveBottom => {
                if index < colors.len() - 1 {
                    let color = colors.remove(index);
                    colors.push(color);
                }
            }
            VecOp::Duplicate => {
                let mut color = colors[index].clone();
                // in order not to make the copied one's name too long
                color.name = get_copy_name(&color.name);
                colors.insert(index, color);
            }
            VecOp::Delete => {
                colors.remove(index);
            }
        }
    }
}

fn get_copy_name(origin: &String) -> String {
    let re = regex::Regex::new(r" #(\d+)$").unwrap();
    match re.captures(origin) {
        Some(caps) => {
            let num: i32 = caps[1].parse().unwrap();
            let new_num = num + 1;
            return re
                .replace(origin, format!(" #{}", new_num).as_str())
                .to_string();
        }
        None => return format!("{} #0", origin),
    }
}
