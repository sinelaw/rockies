use image::GenericImageView;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("png.rs");

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(dest_path)?;

    for entry in fs::read_dir("pngs")? {
        // Load the PNG image
        let file = entry.unwrap();

        let img = image::open(file.path())?;

        // Get the image dimensions
        let (width, height) = img.dimensions();

        let name = file.file_name();
        let name = name.to_str().unwrap();
        let name = name.replace(".png", "");
        println!("{name}");
        let count = width * height;
        output_file.write_all(
            format!("const {name} : [Color; {count}] /* {width} * {height} */ = [").as_bytes(),
        )?;
        // Iterate over the image pixels and add them to the array
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                output_file
                    .write_all(format!("Color {{ r: {r:?}, g: {g:?}, b: {b:?} }},\n").as_bytes())?;
            }
        }
        output_file.write_all(("];").as_bytes())?;
    }

    Ok(())
}
