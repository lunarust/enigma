use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

use common::*;
use crate::pages::rotors::RotorsDisplay;
use crate::pages::reflector::ReflectorDisplay;
use crate::pages::logs::LogsDisplay;
use log::info;

//#[derive(Clone, Copy, PartialEq)]
//enum RotorPos { Slow, Medium, Fast }

#[function_component]
pub fn Home() -> Html {
    let message = use_state(|| "".to_string());
    let full_logs = use_state(|| None);

    let resp_state = use_state(|| (
        "".to_string(),
        "".to_string()
    ));

    let selected_slow_rotor = use_state(|| None);
    let selected_medium_rotor = use_state(|| None);
    let selected_fast_rotor = use_state(|| None);
    let selected_reflector = use_state(|| None);

    let on_reflector_select = {
        let refl = selected_reflector.clone();

        Callback::from(move |reflector: Reflector| {
            refl.set(Some(reflector));
        })
    };

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
    let refl = selected_reflector.clone();
    let full_logs = full_logs.clone();

    let get_responses = {
        let resp_state = resp_state.clone();
        let message = message.clone();
        let full_logs = full_logs.clone();

        Callback::from(move |_| {

            let refl = refl.clone();
            let slow = slow.clone();
            let fast = fast.clone();
            let medium = medium.clone();
            //info!("SLOW {:?}", rotors_array);

            let message = message.clone();
            let (plain, cryptic) = (*resp_state).clone();
            let resp_state = resp_state.clone();
            let full_logs = full_logs.clone();
            spawn_local(async move {

                let json_data = serde_json::json!({
                    "rotor": [
                        slow.as_ref().unwrap_or(&CipherRotor::default()),
                        medium.as_ref().unwrap_or(&CipherRotor::default()),
                        fast.as_ref().unwrap_or(&CipherRotor::default())],
                    "plain": plain,
                    "cryptic": cryptic,
                    "reflector": refl.as_ref().unwrap_or(&Reflector::default()) });
                    info!("Json {:?}", json_data);
                let response = Request::post("/api/v1/scrumble")
                    .header("Content-Type", "application/json")
                    .body(json_data.to_string()).expect("DRAMA")
                    .send().await;

                match response {
                    Ok(resp) if resp.ok() => {
                         let crep_state: common::Response = resp.json().await.unwrap();
                         message.set(format!("{:?}", resp).into());
                         full_logs.set(Some(crep_state.debug_logs));
                         //message.set(format!("Successful {:?} -> crep: {:?}", resp.plain, resp.cryptic, crep_state).into());
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
            <span id="navigation">
            <h3>{"Rotors"}</h3>
            <h4>{"FAST: Right position"}</h4>
            <RotorsDisplay
                on_click={
                    let cb = on_rotor_select.clone();
                    cb.reform(|rotor| ("slow".to_string(), rotor))
                    //cb.reform(|rotor| (RotorPos::Slow, rotor))
                }
            />
            <hr />
            <h4>{"MEDIUM: Middle position"}</h4>
            <RotorsDisplay
                on_click={
                    let cb = on_rotor_select.clone();
                    cb.reform(|rotor| ("medium".to_string(), rotor))
                }
             />
             <hr />
             <h4>{"SLOW: Left position"}</h4>
             <RotorsDisplay
                on_click={
                    let cb = on_rotor_select.clone();
                    cb.reform(|rotor| ("fast".to_string(), rotor))
                }
             />
             <hr />
             <h3>{"Reflector:"}</h3>
             <ReflectorDisplay
                on_click={on_reflector_select}
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

                <div id="logs">

                if let Some(full_logs) = &*full_logs {
                    <LogsDisplay my_logs={ full_logs.clone() } />
                }
                </div>

            </div>


            if !message.is_empty() {
            <p class="footer">{ &*message }</p>
            }
        </div>
    }
}
