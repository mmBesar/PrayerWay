use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Define the assets to copy
    let assets = [
        ("assets/materialdesignicons.ttf", "materialdesignicons.ttf"),
        ("assets/notification.ogg", "notification.ogg"),
    ];

    // Copy each asset to the output directory
    for (source, target) in &assets {
        let dest = Path::new(&out_dir).join(target);
        fs::copy(source, dest).expect(&format!("Failed to copy {} to {}", source, target));
    }

    // Rebuild if any assets are changed
    println!("cargo:rerun-if-changed=assets/materialdesignicons.ttf");
    println!("cargo:rerun-if-changed=assets/notification.ogg");
}
