use std::fs;
use std::path::{Path, PathBuf};
#[cfg(windows)]
use winres::WindowsResource;

fn main() {
    // Ensure rebuilds when inputs change
    println!("cargo:rerun-if-changed=assets/presets");
    println!("cargo:rerun-if-changed=assets/colorlook.ico");

    // Copy assets/presets to exedir/presets
    let assets_dir = Path::new("assets/presets");
    let dest_path = get_output_path().join("presets");

    if !assets_dir.exists() {
        eprintln!(
            "warning: '{}' does not exist; skipping preset copy",
            assets_dir.display()
        );
    } else {
        fs::create_dir_all(&dest_path).expect("failed to create presets output directory");

        for entry in fs::read_dir(assets_dir).expect("failed to read assets/presets directory") {
            let entry = entry.expect("failed to read directory entry");
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().expect("invalid file name");
                let dest_file = dest_path.join(file_name);
                fs::copy(&path, &dest_file).expect("failed to copy preset file");
            }
        }
    }

    // compile resource file
    #[cfg(windows)]
    {
        WindowsResource::new()
            .set_icon("assets/colorlook.ico")
            .compile()
            .expect("failed to compile Windows resources");
    }
}

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let profile = std::env::var("PROFILE").expect("PROFILE not set");
    Path::new(&manifest_dir).join("target").join(profile)
}
