#![windows_subsystem = "windows"]
mod add;
mod color_item;
mod gen;
mod mainwindow;
mod utils;
use eframe::egui;
use eframe::egui::ViewportBuilder;
use std::sync::Arc;

include_flate::flate!(static NERDFONTS: [u8] from "assets/SymbolsNF.ttf");
include_flate::flate!(static HACKFONT: [u8] from "assets/HackNerdFont-Regular.ttf");
include_flate::flate!(static ICON: [u8] from "assets/colorlook.png");

fn main() {
    let icon_img = image::load_from_memory(&ICON).unwrap();
    let icon_buffer = icon_img.to_rgba8();
    let icon_pixels = icon_buffer.as_flat_samples();
    let icon_data = egui::IconData{
        rgba: icon_pixels.to_vec().samples,
        width: icon_img.width(),
        height: icon_img.height(),
    };
    let option = eframe::NativeOptions {
        viewport: ViewportBuilder{
            title: Some("ColorLook".to_string()),
            position: Some(egui::pos2(0f32,0f32)),
            inner_size: Some(egui::Vec2::new(1200f32, 700f32)),
            icon: Some(Arc::new(icon_data)),
            ..Default::default()
        },
        default_theme: eframe::Theme::Dark,
        follow_system_theme: false,
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
