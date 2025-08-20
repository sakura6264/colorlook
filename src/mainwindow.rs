use eframe::egui;

use crate::color_item;
use crate::utils::toast;

const MARGIN: f32 = 40f32;
const TEXTURE_NAME: &str = "bufferimg";

include_flate::flate!(static BUFFER: [u8] from "assets/placeholder.png");

lazy_static::lazy_static! {
    pub static ref PLACEHOLDER: image::DynamicImage = image::load_from_memory(&BUFFER).unwrap();
    static ref TABLIST: Vec<(Tabs,String)> = vec![
        (Tabs::Colors, "\u{e22b} Colors".into()),
        (Tabs::Add, "\u{ea60} Add".into()),
        (Tabs::Gen, "\u{f0674} Generate".into()),
        (Tabs::Preview, "\u{f1205} Preview".into()),
     ];
}

pub struct MainWindow {
    toasts: egui_toast::Toasts,
    file_dialog: FileDialog,
    tab_viewer: MainWindowTabViewer,
    dock_tree: egui_dock::DockState<Tabs>,
}

pub struct MainWindowTabViewer {
    pub colors: Vec<color_item::ColorItem>,
    pub image: image::DynamicImage,
    texture_id: Option<egui::TextureId>,
    pub add_component: Option<Box<dyn crate::add::AddColor>>,
    pub gen_component: Option<Box<dyn crate::gen::Generate>>,
    pub ui_msg: Option<TabMsg>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tabs {
    Colors,
    Add,
    Gen,
    Preview,
}

impl MainWindowTabViewer {
    pub fn new() -> Self {
        return Self {
            colors: Vec::new(),
            image: PLACEHOLDER.clone(),
            texture_id: None,
            add_component: None,
            gen_component: None,
            ui_msg: None,
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
    pub fn ensure_texture(&mut self, ctx: &egui::Context) {
        if self.texture_id.is_none() {
            self.update_texture(ctx);
        }
    }
}

impl egui_dock::TabViewer for MainWindowTabViewer {
    type Tab = Tabs;
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tabs::Colors => "\u{e22b} Colors".into(),
            Tabs::Add => match &self.add_component {
                Some(component) => component.get_name().into(),
                None => "\u{ea60} Add".into(),
            },
            Tabs::Gen => match &self.gen_component {
                Some(component) => component.get_name().into(),
                None => "\u{f0674} Generate".into(),
            },
            Tabs::Preview => "\u{eb28} Preview".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let size = ui.available_size_before_wrap();
        let width = size.x;
        let height = size.y;

        match tab {
            Tabs::Colors => {
                ui.vertical(|ui| {
                    color_item::draw_color_items(ui, &mut self.colors);
                });
            }
            Tabs::Add => {
                ui.vertical(|ui| match self.add_component {
                    Some(ref mut component) => {
                        if let Some(color) = component.paint_ui(ui, &self.image) {
                            self.ui_msg = Some(TabMsg::Add(color));
                        }
                    }
                    None => {
                        ui.label("\u{f08a4} No Component Selected.");
                    }
                });
            }
            Tabs::Gen => {
                ui.vertical(|ui| match self.gen_component {
                    Some(ref mut component) => {
                        if let Some(img) = component.paint_ui(ui, &self.colors) {
                            self.ui_msg = Some(TabMsg::Gen(img));
                        }
                    }
                    None => {
                        ui.label("\u{f08a4} No Component Selected.");
                    }
                });
            }
            Tabs::Preview => {
                if let Some(id) = self.texture_id {
                    ui.add(
                        egui::Image::from_texture(egui::load::SizedTexture::new(
                            id,
                            [self.image.width() as f32, self.image.height() as f32],
                        ))
                        .fit_to_exact_size([width - MARGIN, height - MARGIN].into()),
                    );
                }
            }
        }
    }

    fn allowed_in_windows(&self, _tab: &mut Self::Tab) -> bool {
        true
    }
}

impl MainWindow {
    /// Focus on a specific tab, ensuring it's visible in the dock tree
    /// If the tab doesn't exist, it will be added to the dock tree
    fn focus_tab(&mut self, tab: Tabs) {
        // Check if the tab exists in the dock tree
        match self.dock_tree.find_tab(&tab) {
            Some(node_index) => {
                // Tab exists, set it as active
                self.dock_tree.set_active_tab(node_index);
            }
            None => {
                // Tab doesn't exist, add it to the dock tree
                self.dock_tree.push_to_focused_leaf(tab);
            }
        }
    }

    pub fn new() -> Self {
        let mut tree = egui_dock::DockState::new(vec![Tabs::Preview]);
        let [_, b] = tree.main_surface_mut().split_left(
            egui_dock::NodeIndex::root(),
            0.5,
            vec![Tabs::Add, Tabs::Gen],
        );
        let [_, _] = tree
            .main_surface_mut()
            .split_left(b, 0.5, vec![Tabs::Colors]);

        return Self {
            toasts: egui_toast::Toasts::new()
                .anchor(egui::Align2::LEFT_BOTTOM, (MARGIN, -MARGIN))
                .direction(egui::Direction::BottomUp),
            file_dialog: FileDialog::None,
            tab_viewer: MainWindowTabViewer::new(),
            dock_tree: tree,
        };
    }
}

#[derive(Clone, Copy)]
pub enum MsgFile {
    Load,
    Clear,
    Save,
    Exit,
}

#[derive(Clone, Copy)]
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
    AdjustTab(Tabs),
}

#[derive(Clone)]
pub enum TabMsg {
    Add(Vec<color_item::ColorItem>),
    Gen(image::DynamicImage),
}

pub enum FileDialog {
    None,
    LoadImg(egui_file::FileDialog),
    SaveImg(egui_file::FileDialog),
    ExportJson(egui_file::FileDialog),
    ImportJson(egui_file::FileDialog),
}
impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // call once at the first frame
        self.tab_viewer.ensure_texture(ctx);
        let height = ctx.available_rect().height();
        let width = ctx.available_rect().width();
        // manage message. No One can click 2 buttons in one frame.
        let mut ui_msg = None;
        //register shortcut
        let openshortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::O);
        let saveshortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::S);
        let clearshortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::C);
        let exitshortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::Q);
        let openshortcuttext = ctx.format_shortcut(&openshortcut);
        let saveshortcuttext = ctx.format_shortcut(&saveshortcut);
        let clearshortcuttext = ctx.format_shortcut(&clearshortcut);
        let exitshortcuttext = ctx.format_shortcut(&exitshortcut);
        if ctx.input(|is| is.clone().consume_shortcut(&openshortcut)) {
            ui_msg = Some(Msg::File(MsgFile::Load));
        }
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
                    if ui
                        .add(egui::Button::new("\u{f0214} Load").shortcut_text(openshortcuttext))
                        .clicked()
                    {
                        ui_msg = Some(Msg::File(MsgFile::Load));
                    }
                    if ui
                        .add(egui::Button::new("\u{f1604} Clear").shortcut_text(saveshortcuttext))
                        .clicked()
                    {
                        ui_msg = Some(Msg::File(MsgFile::Clear));
                    }
                    if ui
                        .add(egui::Button::new("\u{f0193} Save").shortcut_text(clearshortcuttext))
                        .clicked()
                    {
                        ui_msg = Some(Msg::File(MsgFile::Save));
                    }
                    if ui
                        .add(egui::Button::new("\u{f05fc} Exit").shortcut_text(exitshortcuttext))
                        .clicked()
                    {
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
                ui.menu_button("\u{eae4} Window", |ui| {
                    let getlabel = |tab, text| {
                        if self.dock_tree.find_tab(tab).is_some() {
                            return egui::RichText::new(text).color(egui::Color32::YELLOW);
                        } else {
                            return egui::RichText::new(text);
                        }
                    };
                    for (tab, text) in TABLIST.iter() {
                        if ui.button(getlabel(tab, text)).clicked() {
                            ui_msg = Some(Msg::AdjustTab(tab.clone()));
                        }
                    }
                });
                ui.menu_button("\u{ea60} Add", |ui| {
                    for (name, component) in crate::add::NAMELIST.iter() {
                        if ui.button(name).clicked() {
                            self.tab_viewer.add_component =
                                Some(crate::add::get_component(component.clone()));
                            // Focus on the Add tab when component changes
                            self.focus_tab(Tabs::Add);
                        }
                    }
                });
                ui.menu_button("\u{f0674} Generate", |ui| {
                    for (name, component) in crate::gen::NAMELIST.iter() {
                        if ui.button(name).clicked() {
                            self.tab_viewer.gen_component =
                                Some(crate::gen::get_component(component.clone()));
                            // Focus on the Generate tab when component changes
                            self.focus_tab(Tabs::Gen);
                        }
                    }
                });
            });
            egui_dock::DockArea::new(&mut self.dock_tree)
                .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
                .show_inside(ui, &mut self.tab_viewer);
        });
        match &mut self.tab_viewer.ui_msg {
            Some(msg) => {
                match msg {
                    TabMsg::Add(color) => {
                        ui_msg = Some(Msg::Add(color.clone()));
                    }
                    TabMsg::Gen(img) => {
                        ui_msg = Some(Msg::Gen(img.clone()));
                    }
                }
                self.tab_viewer.ui_msg = None;
            }
            None => {}
        }
        self.toasts.show(ctx);
        match &mut self.file_dialog {
            FileDialog::LoadImg(dlg) => {
                if dlg.show(ctx).selected() {
                    if let Some(path) = dlg.path() {
                        if let Some(img) = toast::handle_result(
                            image::open(path),
                            format!("Loaded Image from {}", path.display()),
                            "Error loading image",
                            &mut self.toasts,
                        ) {
                            self.tab_viewer.image = img;
                            self.tab_viewer.update_texture(ctx);
                        }
                    }
                }
            }
            FileDialog::SaveImg(dlg) => {
                if dlg.show(ctx).selected() {
                    if let Some(path) = dlg.path() {
                        toast::handle_result(
                            self.tab_viewer.image.save(path),
                            format!("Saved PNG to {}", path.display()),
                            "Error saving image",
                            &mut self.toasts,
                        );
                    }
                }
            }
            FileDialog::ExportJson(dlg) => {
                if dlg.show(ctx).selected() {
                    if let Some(path) = dlg.path() {
                        // Handle the two different error types separately
                        let result = serde_json::to_string(&self.tab_viewer.colors)
                            .map_err(|e| format!("JSON serialization error: {}", e))
                            .and_then(|json_str| {
                                std::fs::write(path, json_str)
                                    .map_err(|e| format!("File write error: {}", e))
                            });

                        toast::handle_result(
                            result,
                            format!("Exported JSON to {}", path.display()),
                            "Error writing JSON",
                            &mut self.toasts,
                        );
                    }
                }
            }
            FileDialog::ImportJson(dlg) => {
                if dlg.show(ctx).selected() {
                    if let Some(path) = dlg.path() {
                        let result = std::fs::read_to_string(path).and_then(|content| {
                            serde_json::from_str::<Vec<color_item::ColorItem>>(&content)
                                .map_err(|e| e.into())
                        });

                        if let Some(colors) = toast::handle_result(
                            result,
                            format!("Imported JSON from {}", path.display()),
                            "Error reading JSON",
                            &mut self.toasts,
                        ) {
                            self.tab_viewer.colors.extend(colors);
                        }
                    }
                }
            }
            FileDialog::None => {}
        }
        if let Some(msg) = ui_msg {
            match msg {
                Msg::File(msg) => match msg {
                    MsgFile::Load => {
                        let mut dialog = egui_file::FileDialog::open_file(None)
                            .title("Load Image")
                            .default_size(egui::vec2(width / 2f32, height - 2f32 * MARGIN))
                            .current_pos(egui::pos2(width / 4f32, MARGIN));
                        dialog.open();
                        self.file_dialog = FileDialog::LoadImg(dialog);
                    }
                    MsgFile::Clear => {
                        self.tab_viewer.image = PLACEHOLDER.clone();
                        self.tab_viewer.update_texture(ctx);
                    }
                    MsgFile::Save => {
                        let mut dialog = egui_file::FileDialog::save_file(None)
                            .title("Save PNG")
                            .default_filename("untitled.png")
                            .filename_filter(Box::new(|name| name.ends_with(".png")))
                            .default_size(egui::vec2(width / 2f32, height - 2f32 * MARGIN))
                            .current_pos(egui::pos2(width / 4f32, MARGIN));
                        dialog.open();
                        self.file_dialog = FileDialog::SaveImg(dialog);
                    }
                    MsgFile::Exit => {
                        std::process::exit(0);
                    }
                },
                Msg::Color(msg) => match msg {
                    MsgColor::Clear => {
                        self.tab_viewer.colors.clear();
                    }
                    MsgColor::Reverse => {
                        self.tab_viewer.colors.reverse();
                    }
                    MsgColor::SortByName => {
                        self.tab_viewer.colors.sort_by(|a, b| a.name.cmp(&b.name));
                    }
                    MsgColor::SortByR => {
                        self.tab_viewer.colors.sort_by(|a, b| a.r.cmp(&b.r));
                    }
                    MsgColor::SortByG => {
                        self.tab_viewer.colors.sort_by(|a, b| a.g.cmp(&b.g));
                    }
                    MsgColor::SortByB => {
                        self.tab_viewer.colors.sort_by(|a, b| a.b.cmp(&b.b));
                    }
                    MsgColor::SortByH => {
                        self.tab_viewer
                            .colors
                            .sort_by(|a, b| a.get_h().total_cmp(&b.get_h()));
                    }
                    MsgColor::SortByS => {
                        self.tab_viewer
                            .colors
                            .sort_by(|a, b| a.get_s().total_cmp(&b.get_s()));
                    }
                    MsgColor::SortByV => {
                        self.tab_viewer
                            .colors
                            .sort_by(|a, b| a.get_v().total_cmp(&b.get_v()));
                    }
                    MsgColor::Import => {
                        let mut dialog = egui_file::FileDialog::open_file(None)
                            .title("Import JSON")
                            .filename_filter(Box::new(|name| name.ends_with(".json")))
                            .default_size(egui::vec2(width / 2f32, height - 2f32 * MARGIN))
                            .current_pos(egui::pos2(width / 4f32, MARGIN));
                        dialog.open();
                        self.file_dialog = FileDialog::ImportJson(dialog);
                    }
                    MsgColor::Export => {
                        let mut dialog = egui_file::FileDialog::save_file(None)
                            .title("Export JSON")
                            .default_filename("untitled.json")
                            .filename_filter(Box::new(|name| name.ends_with(".json")))
                            .default_size(egui::vec2(width / 2f32, height - 2f32 * MARGIN))
                            .current_pos(egui::pos2(width / 4f32, MARGIN));
                        dialog.open();
                        self.file_dialog = FileDialog::ExportJson(dialog);
                    }
                },
                Msg::Add(color) => {
                    for i in color {
                        self.tab_viewer.colors.push(i);
                    }
                }
                Msg::Gen(img) => {
                    self.tab_viewer.image = img;
                    self.tab_viewer.update_texture(ctx);
                }
                Msg::AdjustTab(tab) => match self.dock_tree.find_tab(&tab) {
                    Some(index) => {
                        self.dock_tree.remove_tab(index);
                    }
                    None => {
                        self.dock_tree.add_window(vec![tab]);
                    }
                },
            }
        }
    }
}
