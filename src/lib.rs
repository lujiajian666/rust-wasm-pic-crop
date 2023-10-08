mod utils;

use image::{imageops, GenericImageView, ImageBuffer, RgbImage};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, clip-pic!");
}

#[wasm_bindgen]
pub fn clip() {
    let mut img: RgbImage = ImageBuffer::new(512, 512);
    let subimg = imageops::crop(&mut img, 0, 0, 100, 100);
    assert!(subimg.dimensions() == (100, 100));
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello from Rust!"));

    body.append_child(&val)?;

    Ok(())
}
