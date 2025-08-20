use crate::color_item;
use eframe::egui;

const MARGIN: f32 = 10f32;

pub struct Preset {
    colorlist: Vec<(String, Vec<color_item::ColorItem>)>,
    msg: Option<String>,
    selected_preset: usize,
    search_text: String,
    selected_index: usize,
}

impl Preset {
    pub fn load_colorlist() -> Result<Vec<(String, Vec<color_item::ColorItem>)>, String> {
        //read all json files in exepath/preset
        let exe_path = std::env::current_exe()
            .or(Err("Error Get EXE Path".to_string()))?
            .parent()
            .ok_or("No Parent Dir".to_string())?
            .to_path_buf();
        let preset_path = exe_path.join("presets");
        let mut colorlist = Vec::new();
        for entry in std::fs::read_dir(preset_path).or(Err("Error Read Directory".to_string()))? {
            let entry = entry.or(Err("Error Read Entry".to_string()))?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path
                    .file_name()
                    .ok_or("No File Name")?
                    .to_str()
                    .ok_or("Error Encoding")?
                    .to_string();
                if file_name.ends_with(".json") {
                    let name = file_name[0..file_name.len() - 5].to_string();
                    let json =
                        std::fs::read_to_string(path).or(Err("Error Read String".to_string()))?;
                    let colorlist_json: Vec<color_item::ColorItem> =
                        serde_json::from_str(&json).map_err(|e| e.to_string())?;
                    colorlist.push((name, colorlist_json));
                }
            }
        }
        return Ok(colorlist);
    }
    pub fn new() -> Self {
        let (colorlist, msg) = match Self::load_colorlist() {
            Ok(colorlist) => (colorlist, None),
            Err(msg) => {
                println!("Error: {}", msg);
                (Vec::new(), Some(msg))
            }
        };

        Self {
            colorlist,
            msg,
            selected_preset: 0,
            search_text: "".into(),
            selected_index: 0,
        }
    }
    pub fn show_color(
        ui: &mut egui::Ui,
        color: &color_item::ColorItem,
        select: &String,
    ) -> (bool, bool, egui::Response) {
        // return (is_selected, is_clicked, response for scroll)
        let mut add = false;
        let mut selected = false;
        let response = ui
            .vertical(|ui| {
                let response = ui
                    .horizontal(|ui| {
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(20f32, ui.text_style_height(&egui::TextStyle::Body)),
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
                        // detect selected text and highlight it into yellow
                        let text = color.name.clone();
                        if select.is_empty() {
                            ui.label(egui::RichText::new(text));
                        } else {
                            if let Ok(reg) = regex::Regex::new(select) {
                                if let Some(mat) = reg.find(&text) {
                                    let match_start = mat.start();
                                    let match_end = mat.end();
                                    let len = text.len();
                                    let style = ui.style();
                                    let mut job = egui::text::LayoutJob::default();
                                    egui::RichText::new(&text[0..match_start])
                                        .color(egui::Color32::YELLOW)
                                        .append_to(
                                            &mut job,
                                            style,
                                            egui::FontSelection::Default,
                                            egui::Align::Center,
                                        );
                                    egui::RichText::new(&text[match_start..match_end])
                                        .color(egui::Color32::GREEN)
                                        .append_to(
                                            &mut job,
                                            style,
                                            egui::FontSelection::Default,
                                            egui::Align::Center,
                                        );
                                    egui::RichText::new(&text[match_end..len])
                                        .color(egui::Color32::YELLOW)
                                        .append_to(
                                            &mut job,
                                            style,
                                            egui::FontSelection::Default,
                                            egui::Align::Center,
                                        );
                                    ui.label(job);
                                    selected = true;
                                } else {
                                    ui.label(text);
                                }
                            } else {
                                ui.label(text);
                            }
                        }
                        ui.separator();
                        ui.label(
                            egui::RichText::new(&color.get_hex())
                                .color(color.get_full_value_color32()),
                        );
                        return response;
                    })
                    .inner;
                ui.horizontal(|ui| {
                    add = ui.button("\u{ea60} Add").clicked();
                    if ui.button("\u{ebcc} Hex").clicked() {
                        ui.output_mut(|o| {
                            o.copied_text = color.get_hex();
                        });
                    }
                    if ui.button("\u{ebcc} RGB").clicked() {
                        ui.output_mut(|o| {
                            o.copied_text = format!("{},{},{}", color.r, color.g, color.b);
                        });
                    }
                    if ui.button("\u{ebcc} Name").clicked() {
                        ui.output_mut(|o| {
                            o.copied_text = color.name.clone();
                        });
                    }
                });
                return response;
            })
            .inner;
        return (add, selected, response);
    }
}

impl super::AddColor for Preset {
    fn get_name(&self) -> String {
        return "\u{eae6} Preset Color".into();
    }
    fn paint_ui(
        &mut self,
        ui: &mut egui::Ui,
        _buffer: &image::DynamicImage,
    ) -> Option<Vec<color_item::ColorItem>> {
        let mut focused = false;
        if self.colorlist.is_empty() {
            ui.label("No preset color found.");
            if let Some(msg) = &self.msg {
                ui.label(format!("Error: {}", msg));
            }
            return None;
        }
        ui.horizontal(|ui| {
            ui.label("From: ");
            egui::ComboBox::from_label(" Preset")
                .selected_text(self.colorlist[self.selected_preset].0.clone())
                .show_ui(ui, |ui| {
                    for i in 0..self.colorlist.len() {
                        ui.selectable_value(
                            &mut self.selected_preset,
                            i,
                            self.colorlist[i].0.clone(),
                        );
                    }
                });
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("\u{ea6d} Find: ");
            if ui.button("\u{eab7}").clicked() {
                focused = true;
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            if ui.button("\u{eab4}").clicked() {
                focused = true;
                self.selected_index += 1;
            }
            if ui.text_edit_singleline(&mut self.search_text).changed() {
                self.selected_index = 0;
                focused = true;
            }
        });
        ui.separator();
        let mut colorvec = Vec::new();
        let mut cursor = ui.cursor();
        let size = ui.available_size();
        cursor.set_height(size.y - MARGIN);
        cursor.set_width(size.x);
        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(cursor), |ui| {
            egui::ScrollArea::new([false, true])
                .scroll_bar_visibility(
                    egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible,
                )
                .min_scrolled_height(size.y - MARGIN)
                .id_salt("preset_list")
                .show(ui, |ui| {
                    let mut selected_vec = Vec::new();
                    let mut size = ui.available_size();
                    size.y = 10f32;
                    ui.add_sized(size, egui::Label::new("\u{eb17} Colors"));
                    for i in 0..self.colorlist[self.selected_preset].1.len() {
                        let color = &self.colorlist[self.selected_preset].1[i];
                        let (add, selected, resp) = Self::show_color(ui, color, &self.search_text);
                        if add {
                            colorvec.push(color.clone());
                        }
                        if selected {
                            selected_vec.push(resp);
                        }
                    }
                    if !selected_vec.is_empty() {
                        if self.selected_index >= selected_vec.len() {
                            self.selected_index = selected_vec.len() - 1;
                            if focused {
                                selected_vec[selected_vec.len() - 1]
                                    .scroll_to_me(Some(egui::Align::Center));
                            }
                        } else {
                            if focused {
                                selected_vec[self.selected_index]
                                    .scroll_to_me(Some(egui::Align::Center));
                            }
                        }
                    } else {
                        self.selected_index = 0;
                    }
                });
        });
        if colorvec.is_empty() {
            return None;
        }
        return Some(colorvec);
    }
}
