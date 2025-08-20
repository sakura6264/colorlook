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

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ColorItem {
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
impl ColorItem {
    /// Creates a new ColorItem from RGB values
    pub fn new(name: impl Into<String>, r: u8, g: u8, b: u8) -> Self {
        Self {
            name: name.into(),
            r,
            g,
            b,
        }
    }

    /// Creates a new ColorItem from RGB values in 0.0-1.0 range
    pub fn from_rgb_f32(name: impl Into<String>, r: f32, g: f32, b: f32) -> Self {
        Self {
            name: name.into(),
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
    pub fn get_hex(&self) -> String {
        return format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b);
    }

    /// Creates a ColorItem from a hex color string (with or without #)
    /// Returns None if the hex string is invalid
    pub fn from_hex(hex: &str, name: impl Into<String>) -> Option<Self> {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 6 {
            return None;
        }

        // Use u8::from_str_radix for more robust hex parsing
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some(Self {
            name: name.into(),
            r,
            g,
            b,
        })
    }

    /// Creates a ColorItem from HSV values
    /// - h: Hue in degrees (0-360)
    /// - s: Saturation (0.0-1.0)
    /// - v: Value (0.0-1.0)
    pub fn from_hsv(h: f32, s: f32, v: f32, name: impl Into<String>) -> Self {
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
            name: name.into(),
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        }
    }

    // Helper methods str2u8 and char2u8 removed as they're no longer needed
    // We now use the standard library's u8::from_str_radix for hex parsing
    /// Get hue component (0-360 degrees)
    pub fn get_h(&self) -> f32 {
        let max = self.r.max(self.g).max(self.b);
        let min = self.r.min(self.g).min(self.b);

        if max == min {
            return 0.0; // Achromatic (gray)
        }

        let delta = max - min;

        let h = if max == self.r {
            // Between yellow and magenta
            60.0 * (((self.g as f32 - self.b as f32) / delta as f32) % 6.0)
        } else if max == self.g {
            // Between cyan and yellow
            60.0 * ((self.b as f32 - self.r as f32) / delta as f32 + 2.0)
        } else {
            // Between magenta and cyan
            60.0 * ((self.r as f32 - self.g as f32) / delta as f32 + 4.0)
        };

        if h < 0.0 {
            h + 360.0
        } else {
            h
        }
    }

    /// Get saturation component (0.0-1.0)
    pub fn get_s(&self) -> f32 {
        let max = self.r.max(self.g).max(self.b);

        if max == 0 {
            0.0 // Avoid division by zero
        } else {
            let min = self.r.min(self.g).min(self.b);
            let delta = max - min;
            delta as f32 / max as f32
        }
    }

    /// Get value component (0.0-1.0)
    pub fn get_v(&self) -> f32 {
        self.r.max(self.g).max(self.b) as f32 / 255.0
    }

    /// Get HSV components as a tuple (h, s, v)
    pub fn to_hsv(&self) -> (f32, f32, f32) {
        (self.get_h(), self.get_s(), self.get_v())
    }
    /// Get a full value version of the color (maximum brightness)
    pub fn get_full_value_color32(&self) -> egui::Color32 {
        let max = self.r.max(self.g).max(self.b);
        if max == 0 {
            return egui::Color32::WHITE; // Avoid division by zero
        }

        let scale = 255.0 / max as f32;
        egui::Color32::from_rgb(
            (self.r as f32 * scale) as u8,
            (self.g as f32 * scale) as u8,
            (self.b as f32 * scale) as u8,
        )
    }
    /// Convert to egui::Color32
    pub fn to_color32(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.r, self.g, self.b)
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

/// Generate a name for a copied color item
/// If the original name ends with ' #n' where n is a number, increment n
/// Otherwise, append ' #1' to the name
fn get_copy_name(origin: &str) -> String {
    // Use a lazy_static regex to avoid recompiling it every time
    lazy_static::lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r" #(\d+)$").unwrap();
    }

    if let Some(caps) = RE.captures(origin) {
        // If we have a match, try to parse the number and increment it
        if let Ok(num) = caps[1].parse::<u32>() {
            // Replace the matched pattern with the incremented number
            return RE.replace(origin, format!(" #{}", num + 1)).into_owned();
        }
    }

    // If no match or parsing failed, append ' #1' (more intuitive than #0)
    format!("{} #1", origin)
}
