
use eframe::egui;

use crate::color_item;

const MARGIN: f32 = 40f32;
const TEXTURE_NAME: &str = "bufferimg";

include_flate::flate!(static BUFFER: [u8] from "assets/placeholder.png");


pub enum Tool {
    Add(Box<dyn crate::add::AddColor>),
    Gen(Box<dyn crate::gen::Generate>),
    None
}


pub struct MainWindow {
    colors: Vec<color_item::ColorItem>,
    image: image::DynamicImage,
    texture_id: Option<egui::TextureId>,
    tool: Tool,
}

impl MainWindow {
    pub fn new() -> Self {
        let placeholder = image::load_from_memory(&BUFFER).unwrap();

        return Self {
            colors: Vec::new(),
            image: placeholder,
            texture_id: None,
            tool: Tool::None,
        };
    }

    pub fn update_texture(&mut self, ctx: &egui::Context) {
        let manager = ctx.tex_manager();
        if let Some(id) = self.texture_id {
            manager.write().free(id);
        };
        let size = [self.image.width() as _, self.image.height() as _];
        let rgba = self.image.to_rgba8();
        let pixels = rgba.as_flat_samples();
        let colorimg = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        self.texture_id = Some(manager.write().alloc(
            TEXTURE_NAME.to_string(),
            colorimg.into(),
            egui::TextureOptions::default(),
        ));
    }
}


#[derive(Clone,Copy)]
pub enum MsgFile {
    Clear,
    Save,
    Exit,
}

#[derive(Clone,Copy)]
pub enum MsgColor {
    Clear,
    Reverse,
    SortByName,
    SortByR,
    SortByG,
    SortByB,
    SortByH,
    SortByS,
    SortByV,
    Import,
    Export,
}

#[derive(Clone)]
pub enum Msg {
    File(MsgFile),
    Color(MsgColor),
    Add(Vec<color_item::ColorItem>),
    Gen(image::DynamicImage),
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.texture_id.is_none() {
            self.update_texture(ctx);
        }
        let height = ctx.available_rect().height();
        let width = ctx.available_rect().width();
        let mut ui_msg = None;
        //register shortcut
        let saveshortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::S);
        let clearshortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::C);
        let exitshortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Q);
        let saveshortcuttext = ctx.format_shortcut(&saveshortcut);
        let clearshortcuttext = ctx.format_shortcut(&clearshortcut);
        let exitshortcuttext = ctx.format_shortcut(&exitshortcut);
        if ctx.input(|is| is.clone().consume_shortcut(&saveshortcut)) {
            ui_msg = Some(Msg::File(MsgFile::Save));
        }
        if ctx.input(|is| is.clone().consume_shortcut(&clearshortcut)) {
            ui_msg = Some(Msg::File(MsgFile::Clear));
        }
        if ctx.input(|is| is.clone().consume_shortcut(&exitshortcut)) {
            ui_msg = Some(Msg::File(MsgFile::Exit));
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("\u{f0214} File", |ui| {
                    if ui.add(egui::Button::new("\u{f1604} Clear").shortcut_text(saveshortcuttext)).clicked() {
                        ui_msg = Some(Msg::File(MsgFile::Clear));
                    }
                    if ui.add(egui::Button::new("\u{f0193} Save").shortcut_text(clearshortcuttext)).clicked() {
                        ui_msg = Some(Msg::File(MsgFile::Save));
                    }
                    if ui.add(egui::Button::new("\u{f05fc} Exit").shortcut_text(exitshortcuttext)).clicked() {
                        ui_msg = Some(Msg::File(MsgFile::Exit));
                    }
                });
                ui.menu_button("\u{e22b} Color", |ui| {
                    if ui.button("\u{f0413} Clear").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::Clear));
                    }
                    if ui.button("\u{eaf5} Reverse").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::Reverse));
                    }
                    if ui.button("\u{f04bb} Sort By Name").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::SortByName));
                    }
                    if ui.button("\u{f1385} Sort By Red").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::SortByR));
                    }
                    if ui.button("\u{f1385} Sort By Green").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::SortByG));
                    }
                    if ui.button("\u{f1385} Sort By Blue").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::SortByB));
                    }
                    if ui.button("\u{f1385} Sort By Hue").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::SortByH));
                    }
                    if ui.button("\u{f1385} Sort By Saturation").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::SortByS));
                    }
                    if ui.button("\u{f1385} Sort By Value").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::SortByV));
                    }
                    if ui.button("\u{f02fa} Import").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::Import));
                    }
                    if ui.button("\u{f0207} Export").clicked() {
                        ui_msg = Some(Msg::Color(MsgColor::Export));
                    }
                });
                ui.menu_button("\u{ea60} Add", |ui| {
                    for (name,component) in crate::add::NAMELIST.iter() {
                        if ui.button(name).clicked() {
                            self.tool = Tool::Add(crate::add::get_component(component.clone()));
                        }
                    }
                });
                ui.menu_button("\u{f0674} Generate", |ui| {
                    for (name,component) in crate::gen::NAMELIST.iter() {
                        if ui.button(name).clicked() {
                            self.tool = Tool::Gen(crate::gen::get_component(component.clone()));
                        }
                    }
                });
            });
            ui.horizontal(|ui| {
                let mut cursor = ui.cursor();
                cursor.set_height(height - MARGIN);
                cursor.set_width(width / 4f32);
                ui.allocate_ui_at_rect(cursor, |ui| {
                    egui::ScrollArea::new([true, true])
                        .scroll_bar_visibility(
                            egui::containers::scroll_area::ScrollBarVisibility::AlwaysVisible,
                        )
                        .min_scrolled_height(height - MARGIN)
                        .id_source("colors")
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                let mut size = ui.available_size();
                                size.y = 10f32;
                                ui.add_sized(size, egui::widgets::Label::new("\u{e22b} Colors"));
                                color_item::draw_color_items(ui, &mut self.colors);
                            });
                        });
                });
                ui.add_sized(
                    [MARGIN / 4f32, height - MARGIN],
                    egui::widgets::Separator::default(),
                );
                cursor = ui.cursor();
                cursor.set_height(height - MARGIN);
                cursor.set_width(width / 4f32);
                ui.allocate_ui_at_rect(cursor, |ui| {
                    egui::ScrollArea::vertical()
                        .min_scrolled_height(height - MARGIN)
                        .id_source("tool")
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                let mut size = ui.available_size();
                                size.y = 10f32;
                                match self.tool {
                                    Tool::Add(ref mut component) => {
                                        ui.add_sized(size, egui::widgets::Label::new(component.get_name()));
                                        if let Some(color) = component.paint_ui(ui) {
                                            ui_msg = Some(Msg::Add(color));
                                        }
                                    },
                                    Tool::Gen(ref mut component) => {
                                        ui.add_sized(size, egui::widgets::Label::new(component.get_name()));
                                        if let Some(img) = component.paint_ui(ui, &self.colors.clone()) {
                                            ui_msg = Some(Msg::Gen(img));
                                        }
                                    },
                                    Tool::None => {
                                        ui.add_sized(size, egui::widgets::Label::new("\u{eae6} Welcome"));
                                    }
                                }
                            });
                        });
                });
                ui.add_sized(
                    [MARGIN / 4f32, height - MARGIN],
                    egui::widgets::Separator::default(),
                );
                if let Some(id) = self.texture_id {
                    ui.add(
                        egui::Image::from_texture(egui::load::SizedTexture::new(
                            id,
                            [self.image.width() as f32, self.image.height() as f32],
                        ))
                        .fit_to_exact_size([width / 2f32 - MARGIN, height - MARGIN].into()),
                    );
                }
            });
        });
        if let Some(msg) = ui_msg {
            match msg {
                Msg::File(msg) => match msg {
                    MsgFile::Clear => {
                        self.image = image::DynamicImage::new_rgb8(
                            (width / 2f32) as _,
                            (height - MARGIN) as _,
                        );
                        self.update_texture(ctx);
                    }
                    MsgFile::Save => {
                        let path = rfd::FileDialog::new()
                            .add_filter("PNG Image", &["png"])
                            .set_title("Save Image")
                            .save_file();
                        if let Some(path) = path {
                            if let Err(e) = self.image.save(path) {
                                simple_message_box::create_message_box(
                                    format!("Error:{e}").as_str(),
                                    "Error",
                                );
                            };
                        };
                    }
                    MsgFile::Exit => {
                        std::process::exit(0);
                    }
                },
                Msg::Color(msg) => match msg {
                    MsgColor::Clear => {
                        self.colors.clear();
                    }
                    MsgColor::Reverse => {
                        self.colors.reverse();
                    }
                    MsgColor::SortByName => {
                        self.colors.sort_by(|a, b| a.name.cmp(&b.name));
                    }
                    MsgColor::SortByR => {
                        self.colors.sort_by(|a, b| a.r.cmp(&b.r));
                    }
                    MsgColor::SortByG => {
                        self.colors.sort_by(|a, b| a.g.cmp(&b.g));
                    }
                    MsgColor::SortByB => {
                        self.colors.sort_by(|a, b| a.b.cmp(&b.b));
                    }
                    MsgColor::SortByH => {
                        self.colors.sort_by(|a, b| a.get_h().total_cmp(&b.get_h()));
                    }
                    MsgColor::SortByS => {
                        self.colors.sort_by(|a, b| a.get_s().total_cmp(&b.get_s()));
                    }
                    MsgColor::SortByV => {
                        self.colors.sort_by(|a, b| a.get_v().total_cmp(&b.get_v()));
                    }
                    MsgColor::Import => {
                        let path = rfd::FileDialog::new()
                            .add_filter("JSON", &["json"])
                            .set_title("Import JSON")
                            .pick_file();
                        if let Some(path) = path {
                            let mut err = None;
                            match std::fs::read_to_string(path) {
                                Ok(str) => {
                                    match serde_json::from_str::<Vec<color_item::ColorItem>>(&str) {
                                        Ok(mut color) => {
                                            self.colors.append(&mut color);
                                        },
                                        Err(e) => {
                                            err = Some(e.to_string());
                                        }
                                    }
                                },
                                Err(e) => {
                                    err = Some(e.to_string());
                                }
                            }
                            if let Some(e) = err {
                                simple_message_box::create_message_box(
                                    format!("Error Read JSON:{e}").as_str(),
                                    "Error",
                                );
                            }
                        }
                    }
                    MsgColor::Export => {
                        let path = rfd::FileDialog::new()
                            .add_filter("JSON", &["json"])
                            .set_title("Export JSON")
                            .save_file();
                        if let Some(path) = path {
                            let mut err = None;
                            match serde_json::to_string(&self.colors) {
                                Ok(str) => {
                                    match std::fs::write(path, str) {
                                        Ok(_) => {},
                                        Err(e) => {
                                            err = Some(e.to_string());
                                        }
                                    }
                                },
                                Err(e) => {
                                    err = Some(e.to_string());
                                }
                            }
                            if let Some(e) = err {
                                simple_message_box::create_message_box(
                                    format!("Error Write JSON:{e}").as_str(),
                                    "Error",
                                );
                            }
                        }
                    }
                },
                Msg::Add(color) => {
                    for i in color {
                        self.colors.push(i);
                    }
                }
                Msg::Gen(img) => {
                    self.image = img;
                    self.update_texture(ctx);
                }
            }
        }
    }
}
