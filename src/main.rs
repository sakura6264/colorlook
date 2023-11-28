#![windows_subsystem = "windows"]
mod add;
mod color_item;
mod gen;
mod mainwindow;
mod utils;
use eframe::egui;

include_flate::flate!(static NERDFONTS: [u8] from "assets/SymbolsNF.ttf");
include_flate::flate!(static HACKFONT: [u8] from "assets/HackNerdFont-Regular.ttf");
include_flate::flate!(static ICON: [u8] from "assets/colorlook.png");

fn main() {
    let option = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::Vec2::new(1200f32, 700f32)),
        follow_system_theme: false,
        icon_data: eframe::IconData::try_from_png_bytes(&ICON).ok(),
        initial_window_pos: Some(egui::pos2(0f32,0f32)),
        ..Default::default()
    };
    eframe::run_native(
        "ColorLook",
        option,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "nerdfonts".to_string(),
                egui::FontData::from_static(&NERDFONTS)
            );
            fonts.font_data.insert(
                "hackfont".to_string(),
                egui::FontData::from_static(&HACKFONT)
            );
            fonts.families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "nerdfonts".to_string());
            fonts.families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("nerdfonts".to_string());
            fonts.families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "hackfont".to_string());
            fonts.families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("hackfont".to_string());
            cc.egui_ctx.set_fonts(fonts);
            Box::new(mainwindow::MainWindow::new())
        }),
    )
    .unwrap();
}
