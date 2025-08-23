#![windows_subsystem = "windows"]
mod add;
mod color_item;
mod gen;
mod mainwindow;
mod utils;
use eframe::egui;
use eframe::egui::ViewportBuilder;
use std::sync::Arc;
use utils::fonts;

include_flate::flate!(static NERDFONTS: [u8] from "assets/SymbolsNF.ttf");
include_flate::flate!(static HACKFONT: [u8] from "assets/HackNerdFont-Regular.ttf");
include_flate::flate!(static ICON: [u8] from "assets/colorlook.png");

fn main() {
    // set environment variable DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1=1 to avoid crash on AMD
    // std::env::set_var("DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1", "1");
    // Create application icon using the utility function
    let icon_data = fonts::create_app_icon(&ICON);
    let option = eframe::NativeOptions {
        viewport: ViewportBuilder {
            title: Some("ColorLook".to_string()),
            position: Some(egui::pos2(0f32, 0f32)),
            inner_size: Some(egui::Vec2::new(1200f32, 700f32)),
            icon: Some(Arc::new(icon_data)),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "ColorLook",
        option,
        Box::new(|cc| {
            // Configure fonts using the utility function
            fonts::configure_fonts(&cc.egui_ctx, &NERDFONTS, &HACKFONT);
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(mainwindow::MainWindow::new()))
        }),
    )
    .unwrap();
}
