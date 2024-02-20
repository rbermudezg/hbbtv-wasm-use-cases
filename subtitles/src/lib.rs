// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
mod subtitles;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}

/*
#[wasm_bindgen(start)]
fn run() {
    log("Hello from Rust!");
}
 */

lazy_static! {
    static ref SUBTITLES: Mutex<subtitles::Subtitles> = Mutex::new(subtitles::Subtitles::new());
}

#[wasm_bindgen]
pub fn parse(xml: &str) {
    let mut subtitles = SUBTITLES.lock().unwrap();
    subtitles.load(xml);
    return;
}

#[wasm_bindgen]
pub fn updateSubtitlesForTimecode(ms: i32) {
    let subtitles = SUBTITLES.lock().unwrap();
    subtitles.update_subtitles_for_ms(ms);
    return;
}

// Our Add function
// wasm-pack requires "exported" functions
// to include #[wasm_bindgen]
#[wasm_bindgen]
pub fn add(a: i32, b: i32, x: &str) -> i32 {
    //let x: String = String::from("Hello, ");
    log(&format!("DONE --- {} --- {}", a, x.len()));
    /*
    fn simple_callback(a: i32, b: &cuepoints::Cuepoint) {
        log("hello world ohhh!");
    }
    let cuepoint = cuepoints::Cuepoint {
        id: 0,
        ms: 0,
        timestopass: 0,
        negativemargin: None,
        positivemargin: None,
        callback: Box::new(simple_callback),
        once: false,
    };
    let mut cuepoints = cuepoints::Cuepoints::new();
    cuepoints.add_cuepoint(cuepoint);
    cuepoints.check_cuepoints(0);
    */
    return 1;
}
