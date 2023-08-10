use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_str: &str) -> String{
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_str).unwrap();

    log(&"Image Decode".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();

    log(&"Image loaded".into());

    img = img.grayscale();

    log(&"Grayscale is applied".into());

    let mut buffer = vec![];

    img.write_to(&mut buffer, Png).unwrap();

    log(&"Image is buffered".into());

    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}", encoded_img
    );

data_url
}