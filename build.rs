use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Copy assets/presets to exedir/presets
    let assets_dir = Path::new("assets/presets");
    let dest_path = get_output_path().join("presets");

    fs::create_dir_all(&dest_path).unwrap();

    for entry in fs::read_dir(&assets_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let dest_file = dest_path.join(path.file_name().unwrap());
            fs::copy(&path, &dest_file).unwrap();
        }
    }

    // compile resource file
    embed_resource::compile("assets/icon.rc", embed_resource::NONE);
}

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = std::env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}
