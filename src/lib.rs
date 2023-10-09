mod utils;
use wasm_bindgen::prelude::*;

pub mod clip;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, clip-pic!");
}

// #[wasm_bindgen]
// pub fn clip() {
//     let mut img: RgbImage = ImageBuffer::new(512, 512);
//     let subimg = imageops::crop(&mut img, 0, 0, 100, 100);
//     assert!(subimg.dimensions() == (100, 100));
// }

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}
