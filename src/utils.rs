pub fn resized_str(name: &String , len: usize) -> String {
    let mut name = name.clone();
    if name.len() > len {
        name.truncate(len);
        name.push_str("...");
    }
    return name;
}