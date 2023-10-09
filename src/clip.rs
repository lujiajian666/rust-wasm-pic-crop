use base64::encode;
use image::{DynamicImage, ImageFormat};
use std::io::{Cursor, Read, Seek, SeekFrom};
use wasm_bindgen::prelude::*;

fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    let img = match image::load_from_memory(_array) {
        Ok(img) => img,
        Err(error) => {
            panic!("load error: {:?}", error)
        }
    };
    return img;
}
fn get_image_as_base64(_img: DynamicImage, format: ImageFormat) -> String {
    // 创建一个内存空间
    let mut c = Cursor::new(Vec::new());
    match _img.write_to(&mut c, format) {
        Ok(c) => c,
        Err(error) => {
            panic!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            )
        }
    };
    c.seek(SeekFrom::Start(0)).expect("seek fail");
    let mut out = Vec::new();
    // 从内存读取数据
    c.read_to_end(&mut out).expect("read_to_end fail");
    // 解码
    let stt = encode(&mut out);
    let together = format!("{}{}", "data:image/png;base64,", stt);
    return together;
}
pub fn append_img(image_src: String) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let val = document.create_element("img")?;
    val.set_attribute("src", &image_src)?;
    val.set_attribute("style", "height: 200px")?;
    body.append_child(&val)?;
    Ok(())
}
pub fn set_dom_attr(selectors: &str, attr: &str, value: String) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let target = document
        .query_selector(selectors)
        .expect("query error")
        .expect("target not find");
    target.set_attribute(attr, value.as_str());
}

#[wasm_bindgen]
pub fn grayscale(_array: &[u8]) -> Result<(), JsValue> {
    let format = image::guess_format(_array).expect("can not read pic format");
    let mut img = load_image_from_array(_array);
    img = img.grayscale();
    let base64_str = get_image_as_base64(img, format);
    return append_img(base64_str);
}

#[wasm_bindgen]
pub fn crop(
    _array: &[u8],
    dom_select: String,
    attr: String,
    top_left_x: u32,
    top_left_y: u32,
    height: u32,
    width: u32,
) -> Result<(), JsValue> {
    let format = image::guess_format(_array).expect("can not read pic format");
    let mut img = load_image_from_array(_array);
    img = img.crop(top_left_x, top_left_y, width, height);
    let base64_str = get_image_as_base64(img, format);
    set_dom_attr(dom_select.as_str(), attr.as_str(), base64_str);
    Ok(())
}
