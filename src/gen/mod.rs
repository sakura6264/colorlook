use eframe::egui;
use image;

use crate::color_item;

mod line;
mod circle;
mod mono;
mod blocks;

lazy_static::lazy_static! {
    pub static ref NAMELIST: Vec<(String, GenerateComponent)> = get_component_namelist();
}

pub trait Generate {
    fn paint_ui(
        &mut self,
        ui: &mut egui::Ui,
        colors: &Vec<color_item::ColorItem>,
    ) -> Option<image::DynamicImage>;
    fn get_name(&self) -> String;
}

#[derive(Clone, Copy)]
pub enum GenerateComponent {
    Line,
    Circle,
    Mono,
    Blocks,
}

pub fn get_component(component: GenerateComponent) -> Box<dyn Generate> {
    match component {
        GenerateComponent::Line => Box::new(line::Line::new()),
        GenerateComponent::Circle => Box::new(circle::Circle::new()),
        GenerateComponent::Mono => Box::new(mono::Mono::new()),
        GenerateComponent::Blocks => Box::new(blocks::Blocks::new()),
    }
}

pub fn get_component_namelist() -> Vec<(String, GenerateComponent)> {
    let mut list = Vec::new();
    list.push(("\u{f012a} Line".into(), GenerateComponent::Line));
    list.push(("\u{f0e96} Circle".into(), GenerateComponent::Circle));
    list.push(("\u{eae6} Mono".into(), GenerateComponent::Mono));
    list.push(("\u{f0763} Blocks".into(), GenerateComponent::Blocks));
    return list;
}
