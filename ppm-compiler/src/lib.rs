mod compiler;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement};
use wasm_bindgen_futures::JsFuture;
use std::rc::Rc;
use serde_json::json;
use std::cell::RefCell;
use web_sys::console;

#[wasm_bindgen]
pub fn open_file(callback: &js_sys::Function) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let input:HtmlInputElement = document
        .create_element("input")?
        .dyn_into::<HtmlInputElement>()?;

    input.set_type("file");
    document.body().unwrap().append_child(&input)?;
    console::log_1(&format!("Something Comforting!").into());

    let callback_rc = Rc::new(RefCell::new(Some(callback.clone())));
    let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let callback = callback_rc.borrow_mut().take().expect("Callback is missing");
        wasm_bindgen_futures::spawn_local(async move {
            let target = event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();

            let file_list = target.files().unwrap();
            let file = file_list.get(0).unwrap();
            let text = JsFuture::from(file.text())
                .await
                .unwrap()
                .as_string()
                .unwrap();

                
            let this = JsValue::null();
            let _ = match compiler::compile(&text) {
                Ok(result) => callback.call1(&this, &JsValue::from_str(&result)),
                Err(err) => {
                    let result_json = json!({
                        "status": false,
                        "data": err
                    });

                    callback.call1(&this, &JsValue::from_str(&result_json.to_string()))
                },
            };
        })
    });

    input.set_onchange(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    input.click();
    Ok(())
}