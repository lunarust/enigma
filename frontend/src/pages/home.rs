use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

use common::*;
use crate::pages::rotors::RotorsDisplay;

use log::info;

#[derive(Clone, Copy, PartialEq)]
enum RotorPos { Slow, Medium, Fast }

#[function_component]
pub fn Home() -> Html {
    let message = use_state(|| "".to_string());

    let resp_state = use_state(|| (
        "".to_string(),
        "".to_string()
    ));

    let selected_slow_rotor = use_state(|| None);
    let selected_medium_rotor = use_state(|| None);
    let selected_fast_rotor = use_state(|| None);

    let on_rotor_select = {
        let slow = selected_slow_rotor.clone();
        let medium = selected_medium_rotor.clone();
        let fast = selected_fast_rotor.clone();

        Callback::from(move |(name, rotor): (String, CipherRotor)| {
            match name.as_str() {
                "slow" => slow.set(Some(rotor)),
                "medium" => medium.set(Some(rotor)),
                "fast" => fast.set(Some(rotor)),
                _ => (),
            }
        })
    };


    let slow = selected_slow_rotor.clone();
    let medium = selected_medium_rotor.clone();
    let fast = selected_fast_rotor.clone();
    let get_responses = {
        let resp_state = resp_state.clone();
        let message = message.clone();

        Callback::from(move |_| {
            let rotors_array = vec![
                slow.as_ref().unwrap().id,
                medium.as_ref().unwrap().id,
                fast.as_ref().unwrap().id
            ];
            //info!("SLOW {:?}", rotors_array);

            let message = message.clone();
            let (plain, cryptic) = (*resp_state).clone();
            let resp_state = resp_state.clone();

            spawn_local(async move {

                let json_data = serde_json::json!({
                    "rotor": rotors_array,
                    "plain": plain,
                    "cryptic": cryptic,
                    "reflector": "".to_string() });
                let response = Request::post("/api/v1/scrumble")
                    .header("Content-Type", "application/json")
                    .body(json_data.to_string()).expect("DRAMA")
                    .send().await;

                match response {
                    Ok(resp) if resp.ok() => {
                         let crep_state: common::Ciphertext = resp.json().await.unwrap();
                         message.set(format!("Successful {:?} -> crep: {:?}", resp, crep_state).into());
                         resp_state.set((crep_state.plain, crep_state.cryptic))
                    }
                    _ => message.set(format!("Failed {:?}", response).into()),
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "Enigma..." }</h1>
            //if let Some(rotor) = &*selected_medium_rotor { { format!("Medium: {}", &rotor.id) } }
            //if let Some(rotor) = &*selected_fast_rotor { { format!("Fast: {}", &rotor.id) } }

            <span id="navigation">
            <h3>{"Rotors"}</h3>
            <h4>{"Slow:"}</h4>
            <RotorsDisplay
                on_click={
                    let cb = on_rotor_select.clone();
                    cb.reform(|rotor| ("slow".to_string(), rotor))
                    //cb.reform(|rotor| (RotorPos::Slow, rotor))
                }
            />
            <hr />
            <h4>{"Medium:"}</h4>
            <RotorsDisplay
                on_click={
                    let cb = on_rotor_select.clone();
                    cb.reform(|rotor| ("medium".to_string(), rotor))
                }
             />
             <hr />
             <h4>{"Fast:"}</h4>
             <RotorsDisplay
                on_click={
                    let cb = on_rotor_select.clone();
                    cb.reform(|rotor| ("fast".to_string(), rotor))
                }
             />
             </span>

             <div id="content">
                <div id="form">
                 <textarea
                    placeholder="input_plain"
                    value={resp_state.0.clone()}
                    oninput={Callback::from({
                    let resp_state = resp_state.clone();
                    move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                        resp_state.set((input.value(), resp_state.1.clone()));
                    }
                    })}
                 />

                <textarea
                    placeholder="input_cryptic"
                    value={resp_state.1.clone()}
                    oninput={Callback::from({
                    let resp_state = resp_state.clone();
                    move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
                        resp_state.set((resp_state.0.clone(), input.value()));
                    }
                    })}
                />


                <div id="actions">

                <input

                    type="text"
                    value={"bq cr di ej kw mt os px uz gh"}
                    />
                    <br />
                    <button onclick={get_responses.reform(|_| ())} >
                        { "Encrypt" }
                    </button>
                    <button onclick={get_responses.reform(|_| ())} >
                        { "Decrypt" }
                    </button>

                </div>

                </div>
                <br />


            </div>
            if !message.is_empty() {
            <p class="footer">{ &*message }</p>
            }
        </div>
    }
}
