use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

mod connector;
mod app;

const ELEMENT_ID: &str = "yew-wasm-rust-app";

#[wasm_bindgen]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Some info");
    log::error!("Error message");
    //log::info!("Update: {:?}", msg);

    let window = web_sys::window().expect("window to be available.");
    let document = window.document().expect("document to be available");
    let element = document.get_element_by_id(ELEMENT_ID);
    if element.is_none() {
        log::error!("Mount element not found, {:?}", ELEMENT_ID);
    } else {
        log::info!("Mount element found, {:?}", ELEMENT_ID);
    }

    yew::start_app_in_element::<app::Model>(element.expect("expected mount element to exist"));

    let body = document.body().expect("document expect to have have a body");
    let val = body.dyn_into::<web_sys::HtmlBodyElement>().unwrap();
    val.set_bg_color("lightblue");
}
