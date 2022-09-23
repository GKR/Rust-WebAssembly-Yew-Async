use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture};
use web_sys::{
    Request, RequestInit, RequestMode, Response,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Payload {
    pub qotd_date: String,
    pub quote: Quote,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Quote {
    pub id: i32,
    pub dialogue: bool,
    pub private: bool,
    pub tags: Vec<String>,
    pub favorites_count: i32,
    pub upvotes_count: i32,
    pub downvotes_count: i32,
    pub author: String,
    pub author_permalink: String,
    pub body: String,
}

pub async fn fetchit() -> Result<Option<Payload>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("https://favqs.com/api/qotd", &opts).unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);
    let future = JsFuture::from(request_promise);

    let resp = future.await?;
    let resp: Response = resp.dyn_into().expect("response not working...");
    log::info!("Response: {:?}", resp);
    
    if let Ok(json) = resp.json() {
        if let Ok(json) = JsFuture::from(json).await {
            //let js_value = serde_wasm_bindgen::to_value(&json)?
            //let res: Payload = serde_json::from_str(json);
            //if let Ok(res) = json.into_serde::<Payload>() {
            //let _example: Payload = json.into_serde().unwrap();
            
            if let Ok(rv) = json.into_serde::<Payload>() {
                Ok(Some(rv))
            } else {
                Ok(None)
            }
            
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}