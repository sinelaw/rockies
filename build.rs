use image::GenericImageView;
use itertools::Itertools;
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
        .truncate(true)
        .open(dest_path)?;

    output_file.write_all(b"use crate::color::Color;\n")?;
    let read_dir = fs::read_dir("pngs")?;
    let sorted_entries = read_dir
        .map(|x| x.unwrap())
        .sorted_by(|a, b| a.file_name().cmp(&b.file_name()))
        .collect_vec();

    for file in sorted_entries {
        // Load the PNG image

        let img = image::open(file.path())?;

        // Get the image dimensions
        let (width, height) = img.dimensions();

        let name = file.file_name();
        let name = name.to_str().unwrap();
        let name = name.replace(".png", "").to_uppercase();
        println!("{name}");
        let count = width * height;
        output_file.write_all(
            format!("const {name}_IMAGE : [Color; {count}] /* {width} * {height} */ = [")
                .as_bytes(),
        )?;
        // Iterate over the image pixels and add them to the array
        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                output_file
                    .write_all(format!("Color {{ r: {r:?}, g: {g:?}, b: {b:?} }},").as_bytes())?;
            }
        }
        output_file.write_all(("];\n\n").as_bytes())?;
        output_file.write_all( 
            format!("pub const {name} : (usize, usize, &[Color]) = ( {width}, {height}, &{name}_IMAGE );\n").as_bytes())?;
    }

    Ok(())
}
