use std::fs::File;
use std::io::Read;
use image::{self, ImageFormat};

fn main() {
    let mut file = File::open("/tmp/test").expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap_or_else(|e| {
        eprintln!("Failed to file: {}", e);
        std::process::exit(1)
    });

    let im = image::load_from_memory_with_format(&buffer, ImageFormat::Png)
        .unwrap_or_else(|e| {
            eprintln!("Failed to load image from buffer: {}", e);
            std::process::exit(1)
        });

    let rgb = im.to_rgba8().to_vec();

    println!("{:?}", rgb);
}

