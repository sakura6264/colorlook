use eframe::egui;

/// Configure fonts for the application
pub fn configure_fonts(ctx: &egui::Context, nerd_font_data: Vec<u8>, hack_font_data: Vec<u8>) {
    let mut fonts = egui::FontDefinitions::default();

    // Add font data
    fonts.font_data.insert(
        "nerdfonts".to_string(),
        egui::FontData::from_owned(nerd_font_data),
    );

    fonts.font_data.insert(
        "hackfont".to_string(),
        egui::FontData::from_owned(hack_font_data),
    );

    // Configure font families
    // Add nerdfonts to both proportional and monospace families
    add_font_to_family(
        &mut fonts,
        egui::FontFamily::Proportional,
        "nerdfonts",
        true,
    );
    add_font_to_family(&mut fonts, egui::FontFamily::Monospace, "nerdfonts", false);

    // Add hackfont to both proportional and monospace families
    add_font_to_family(&mut fonts, egui::FontFamily::Proportional, "hackfont", true);
    add_font_to_family(&mut fonts, egui::FontFamily::Monospace, "hackfont", false);

    // Apply the font configuration
    ctx.set_fonts(fonts);
}

/// Helper function to add a font to a font family
/// If `insert_at_start` is true, the font will be inserted at the beginning of the list
/// Otherwise, it will be appended to the end
fn add_font_to_family(
    fonts: &mut egui::FontDefinitions,
    family: egui::FontFamily,
    font_name: &str,
    insert_at_start: bool,
) {
    let family_list = fonts.families.entry(family).or_default();

    if insert_at_start {
        family_list.insert(0, font_name.to_string());
    } else {
        family_list.push(font_name.to_string());
    }
}

/// Create an icon for the application window
pub fn create_app_icon(icon_data: &[u8]) -> egui::IconData {
    let icon_img = image::load_from_memory(icon_data).expect("Failed to load icon");
    let icon_buffer = icon_img.to_rgba8();
    let icon_pixels = icon_buffer.as_flat_samples();

    egui::IconData {
        rgba: icon_pixels.to_vec().samples,
        width: icon_img.width(),
        height: icon_img.height(),
    }
}
