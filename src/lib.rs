use image::io::Reader as ImageReader;
// use image::GenericImageView;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

// fn main() {
//     let img = image::open("tmp.png").unwrap();
//     img.save("test.gif").unwrap();
// }

#[wasm_bindgen]
pub fn encode_gif(imgBytes: Vec<u8>) -> Vec<u8> {
    // decode
    let img = ImageReader::new(Cursor::new(imgBytes))
        .with_guessed_format()
        .expect("never fail")
        .decode().unwrap();
    // encode as GIF
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Gif);
    return bytes;
}


