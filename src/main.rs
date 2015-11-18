extern crate image;

use std::path::Path;

use image::GenericImage;

mod stores;
use stores::structs::*;
use stores::stores::*;
fn main() {
    let img = image::open(&Path::new("ins/in1/original.png")).unwrap();

    let (width, height) = img.dimensions();
    let mut vec: Vec<u8> = vec![];
    let mut store: SerialStore = Store::new();

    for (_, _, rgba) in img.pixels() {
        let (r, g, b, _) = (rgba[0], rgba[1], rgba[2], rgba[3]);
        {
            let color = Color{r: r, g: g, b: b};
            store.add_pixel(Box::new(color));
        }
        match store.pop() {
            None => println!("YYYYY"),
            Some(color_box) => {
                let color = *color_box;
                vec.push(color.r);
                vec.push(color.g);
                vec.push(color.b);
            }
        }
    }

    let _ = image::save_buffer(&Path::new("ins/in1/output.png"), &vec, width, height, image::RGB(8));
}
