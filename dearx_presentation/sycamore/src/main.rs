use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(message: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        button(on:click=|_| {  }){
            "Button"
        }
    }
}

fn main() {
    // sycamore で生成する UI
    let node = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("sycamore")
        .unwrap();
    sycamore::render_to(App, &node);

    // ボタンにイベントを登録
    let button = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("my_button")
        .unwrap();
    let closure = Closure::wrap(Box::new(move |_e| {
        async move { invoke("cmd", to_value(&GreetArgs { name: "AAA" }).unwrap()).await };
        log("MyLog");
        // alert("message");
    }) as Box<dyn FnMut(JsValue)>);
    button
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();

    // JS 側で破棄してもらうよう Rust 側のメモリ管理から外す
    closure.forget();
}
