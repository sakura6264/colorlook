use rand::Rng;
pub mod auto_palette;
pub mod fonts;
pub mod toast;

pub fn resized_str(name: &String, len: usize) -> String {
    let mut name = name.clone();
    if name.len() > len {
        name.truncate(len);
        name.push_str("...");
    }
    name
}

pub fn get_random_name(len: usize) -> String {
    let mut rng = rand::rng();
    let mut name = String::new();
    for _ in 0..len {
        let c: char = rng.random_range('a'..='z');
        name.push(c);
    }
    name
}
