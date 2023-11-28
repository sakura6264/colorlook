use eframe::egui;
use rand::Rng;

use crate::color_item;

mod customized;
mod picker;
mod preset;

lazy_static::lazy_static!{
    pub static ref NAMELIST: Vec<(String,AddColorComponent)> = get_component_namelist();
}

pub trait AddColor {
    fn paint_ui(&mut self, ui: &mut egui::Ui) -> Option<Vec<color_item::ColorItem>>;
    fn get_name(&self) -> String;
}

#[derive(Clone, Copy)]
pub enum AddColorComponent {
    Customized,
    Picker,
    Preset,
}

pub fn get_component(component: AddColorComponent) -> Box<dyn AddColor> {
    match component {
        AddColorComponent::Customized => Box::new(customized::Customized::new()),
        AddColorComponent::Picker => Box::new(picker::Picker::new()),
        AddColorComponent::Preset => Box::new(preset::Preset::new()),
    }
}

pub fn get_component_namelist() -> Vec<(String, AddColorComponent)> {
    let mut list = Vec::new();
    list.push(("\u{f03a} Customized".into(), AddColorComponent::Customized));
    list.push(("\u{f0485} Color Picker".into(), AddColorComponent::Picker));
    list.push(("\u{eb9c} Presets".into(), AddColorComponent::Preset));
    return list;
}

pub fn get_random_name(len:usize) -> String {
    let mut rng = rand::thread_rng();
    let mut name = String::new();
    for _ in 0..len {
        let c:char = rng.gen_range('a'..='z');
        name.push(c);
    }
    return name;
}