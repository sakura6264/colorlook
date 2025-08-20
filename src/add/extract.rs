use crate::color_item;
use crate::utils::auto_palette;
use eframe::egui;
use std::sync::mpsc;
use std::thread;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PaletteTheme {
    Vivid,
    Muted,
    Light,
    Dark,
}

pub struct Extract {
    name: String,
    theme: PaletteTheme,
    algo: auto_palette::Algorithm,
    max_color: usize,
    hthread: Option<thread::JoinHandle<()>>,
    channel: Option<mpsc::Receiver<Vec<color_item::ColorItem>>>,
}

impl Extract {
    pub fn new() -> Self {
        Self {
            name: crate::utils::get_random_name(5),
            theme: PaletteTheme::Vivid,
            algo: auto_palette::Algorithm::GMeans,
            max_color: 10,
            hthread: None,
            channel: None,
        }
    }
}

impl super::AddColor for Extract {
    fn get_name(&self) -> String {
        return "\u{eae6} Extract Palette".into();
    }
    fn paint_ui(
        &mut self,
        ui: &mut egui::Ui,
        buffer: &image::DynamicImage,
    ) -> Option<Vec<color_item::ColorItem>> {
        ui.horizontal(|ui| {
            ui.label("\u{f1050} Name:");
            ui.text_edit_singleline(&mut self.name);
        });
        ui.horizontal(|ui| {
            ui.label("\u{eb04} Max Color:");
            ui.add(
                egui::DragValue::new(&mut self.max_color)
                    .speed(0.2)
                    .range(1..=255),
            );
        });
        ui.horizontal(|ui| {
            ui.label("\u{e9d9} Theme:");
            ui.selectable_value(&mut self.theme, PaletteTheme::Vivid, "Vivid");
            ui.selectable_value(&mut self.theme, PaletteTheme::Muted, "Muted");
            ui.selectable_value(&mut self.theme, PaletteTheme::Light, "Light");
            ui.selectable_value(&mut self.theme, PaletteTheme::Dark, "Dark");
        });
        ui.horizontal(|ui| {
            ui.label("\u{e9d9} Algorithm:");
            ui.selectable_value(&mut self.algo, auto_palette::Algorithm::GMeans, "GMeans");
            ui.selectable_value(&mut self.algo, auto_palette::Algorithm::DBSCAN, "DBSCAN")
                .on_hover_text("Slow");
        });
        ui.horizontal(|ui| {
            if ui.button("\u{ea60} Extract").clicked() && self.hthread.is_none() {
                let img = buffer.clone().into_rgb8().into();
                let max_color = self.max_color;
                let basename = self.name.clone();
                let algorithm = self.algo.clone();
                let theme = self.theme.clone();
                let (tx, rx) = mpsc::channel();
                self.channel = Some(rx);
                self.hthread = Some(thread::spawn(move || {
                    let palette: auto_palette::Palette<f64> =
                        auto_palette::Palette::extract_with_algorithm(&img, &algorithm);
                    let swatches = match theme {
                        PaletteTheme::Vivid => {
                            palette.swatches_with_theme(max_color, &auto_palette::Vivid)
                        }
                        PaletteTheme::Muted => {
                            palette.swatches_with_theme(max_color, &auto_palette::Muted)
                        }
                        PaletteTheme::Light => {
                            palette.swatches_with_theme(max_color, &auto_palette::Light)
                        }
                        PaletteTheme::Dark => {
                            palette.swatches_with_theme(max_color, &auto_palette::Dark)
                        }
                    };
                    let mut colors: Vec<color_item::ColorItem> = swatches
                        .iter()
                        .map(|swatch| {
                            let clr = swatch.color().to_rgb();
                            let pos = swatch.position();
                            let pop = swatch.population();
                            let name = format!("{}-({},{})-{}", basename, pos.0, pos.1, pop);
                            let color = color_item::ColorItem {
                                name: name,
                                r: clr.r(),
                                g: clr.g(),
                                b: clr.b(),
                            };
                            color
                        })
                        .collect();
                    colors.dedup();
                    colors.sort_by(|a, b| a.name.cmp(&b.name));
                    tx.send(colors).unwrap();
                }));
            }
            if self.hthread.is_some() {
                ui.spinner();
            }
        });
        if let Some(rx) = &self.channel {
            if let Ok(colors) = rx.try_recv() {
                self.hthread = None;
                self.channel = None;
                self.name = crate::utils::get_random_name(5);
                return Some(colors);
            }
        }
        return None;
    }
}
