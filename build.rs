use image::{GenericImageView, Rgb};
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("png.rs");

    for entry in fs::read_dir("pngs")? {
        // Load the PNG image
        let file = entry.unwrap();

        let img = image::open(file.path())?;

        // Get the image dimensions
        let (width, height) = img.dimensions();

        // Create a new array to store the pixels
        let mut pixels: Vec<Rgb<u8>> = Vec::with_capacity((width * height) as usize);

        // Iterate over the image pixels and add them to the array
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                pixels.push(Rgb([pixel[0], pixel[1], pixel[2]]));
            }
        }
        let name = file.file_name();
        let name = name.to_str().unwrap();
        let name = name.replace(".png", "");
        println!("{name}");
        fs::write(&dest_path, format!("const {name} = {pixels:?};\n"))?;
    }

    Ok(())
}
