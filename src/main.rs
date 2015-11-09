extern crate image;

use std::path::Path;

use image::GenericImage;

fn main() {
    let img = image::open(&Path::new("ins/in1/original.png")).unwrap();
    
    let (width, height) = img.dimensions();
    let mut vec: Vec<u8> = vec![];
    
    for (_, _, rgba) in img.pixels() {
        let (r, g, b, _) = (rgba[0], rgba[1], rgba[2], rgba[3]);
        vec.push(r);
        vec.push(g);
        vec.push(b);
    }

    let _ = image::save_buffer(&Path::new("ins/in1/output.png"), &vec, width, height, image::RGB(8));
}
