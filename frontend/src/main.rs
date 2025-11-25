use yew::prelude::*;
use serde::{ Deserialize, Serialize };
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[function_component(App)]
fn app() -> Html {
    let resp_state = use_state(|| (None as Option<i32>,
        "".to_string(),
        "".to_string()
    ));

    let backend_url = "http://localhost:9000/scrumble";
    let message = use_state(|| "".to_string());

    let crep = common::CiphertextResponse { rotor: 1, plain: "cc".to_string(), cryptic: "aa".to_string() };

    let get_responses = {
        let resp_state = resp_state.clone();
        let message = message.clone();

        Callback::from(move |_| {
            let message = message.clone();
            let (rotor, plain, cryptic) = (*resp_state).clone();
            let resp_state = resp_state.clone();

            spawn_local(async move {

                let json_data = serde_json::json!({ "rotor": 5, "plain": plain, "cryptic": cryptic });
                let response = Request::post(backend_url)
                    .header("Content-Type", "application/json")
                    .body(json_data.to_string()).expect("DRAMA")
                    .send().await;

                match response {
                    Ok(resp) if resp.ok() => {
                         let crep_state: common::CiphertextResponse = resp.json().await.unwrap();
                         message.set(format!("Successful {:?} -> crep: {:?}", resp, crep_state).into());
                         resp_state.set((None, crep_state.plain, crep_state.cryptic))
                    }
                    _ => message.set(format!("Failed {:?}", response).into()),
                }
            });
        })
    };

    html! {
        <>
            <input
                placeholder="input_plain"
                value={resp_state.1.clone()}
                oninput={Callback::from({
                let resp_state = resp_state.clone();
                move |e: InputEvent| {
                    let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                    resp_state.set((resp_state.0, input.value(), resp_state.2.clone()));
                }
                })}
                class="border rounded px-4 py-2 mr-2"
            />
            <button
                onclick={get_responses.reform(|_| ())}
                class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4"
            >
                { "Encrypt" }
            </button>
            <input
                placeholder="input_cryptic"
                value={resp_state.2.clone()}
                oninput={Callback::from({
                let resp_state = resp_state.clone();
                move |e: InputEvent| {
                    let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                    resp_state.set((resp_state.0, resp_state.1.clone(), input.value()));
                }
                })}
                class="border rounded px-4 py-2 mr-2"
            />
            if !message.is_empty() {
            <p class="footer">{ &*message }</p>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
