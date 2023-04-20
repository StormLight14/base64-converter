use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct EncodeArgs<'a> {
    text: &'a str,
}

#[derive(Serialize, Deserialize)]
struct DecodeArgs<'a> {
    text: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let encode_input_ref = use_node_ref();
    let decode_input_ref = use_node_ref();

    let encode_text = use_state(|| String::new());
    let decode_text = use_state(|| String::new());

    let encoded_text = use_state(|| String::new());
    {
        let encoded_text = encoded_text.clone();
        let encode_text = encode_text.clone();
        let encode_text2 = encode_text.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if encode_text.is_empty() {
                        return;
                    }

                    let args = to_value(&EncodeArgs {
                        text: general_purpose::STANDARD
                            .encode(encode_text.as_bytes())
                            .as_str(),
                    })
                    .unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("encode_base64", args).await.as_string().unwrap();
                    encoded_text.set(new_msg);
                });

                || {}
            },
            encode_text2,
        );
    }

    let encode_base64 = {
        let encode_text = encode_text.clone();
        let encode_input_ref = encode_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let input_elem = encode_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap();
            encode_text.set(input_elem.value());

            input_elem.set_value(""); // Clear input field
        })
    };

    let decoded_text = use_state(|| String::new());
    {
        let decoded_text = decoded_text.clone();
        let decode_text = decode_text.clone();
        let decode_text2 = decode_text.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if decode_text.is_empty() {
                        return;
                    }

                    let bytes = general_purpose::STANDARD
                        .decode::<&[u8]>(decode_text.as_ref())
                        .unwrap();

                    let args = to_value(&DecodeArgs {
                        text: String::from_utf8(bytes).unwrap().as_str(),
                    })
                    .unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("decode_base64", args).await.as_string().unwrap();
                    decoded_text.set(new_msg);
                });

                || {}
            },
            decode_text2,
        );
    }

    let decode_base64 = {
        let decode_text = decode_text.clone();
        let decode_input_ref = decode_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let input_elem = decode_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap();
            decode_text.set(input_elem.value());

            input_elem.set_value(""); // Clear input field
        })
    };

    html! {
        <main class="container">
            <h1>{"Base64 converter"}</h1>

            // encode form
            <form class="row" onsubmit={encode_base64}>
                <input class="text-input" ref={encode_input_ref} placeholder="Enter text to encode..." />
                <button type="submit">{"Encode"}</button>
            </form>

            // decode form
            <form class="row" onsubmit={decode_base64}>
                <input class="text-input" ref={decode_input_ref} placeholder="Enter text to decode..." />
                <button type="submit">{"Decode"}</button>
            </form>

            <h2>{"Encoded"}</h2>
            <p><b>{ &*encoded_text }</b></p>
            <h2>{"Decoded"}</h2>
            <p><b>{ &*decoded_text }</b></p>
            <p>{"If decoded text is empty or doesn't change, you entered invalid base64 or some other error occured."}</p>
        </main>
    }
}
