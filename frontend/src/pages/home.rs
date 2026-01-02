use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

use common::*;
use crate::pages::rotors::RotorsDisplay;
use crate::pages::reflector::ReflectorDisplay;
use crate::pages::logs::LogsDisplay;
use crate::pages::configuration::ConfigurationDisplay;
use log::info;

//#[derive(Clone, Copy, PartialEq)]
//enum RotorPos { Slow, Medium, Fast }

#[function_component]
pub fn Home() -> Html {
    let message = use_state(|| "".to_string());
    let full_logs = use_state(|| None);
    let full_conf = use_state(|| None);

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
    let full_conf = full_conf.clone();

    let get_responses = {
        let resp_state = resp_state.clone();
        let message = message.clone();
        let full_logs = full_logs.clone();
        let full_conf = full_conf.clone();

        Callback::from(move |direction: &str| {
            let check = format!("/api/v1/{}", direction);

            let refl = refl.clone();
            let slow = slow.clone();
            let fast = fast.clone();
            let medium = medium.clone();

            let message = message.clone();
            let (plain, cryptic) = (*resp_state).clone();
            let resp_state = resp_state.clone();
            let full_logs = full_logs.clone();
            let full_conf = full_conf.clone();

            spawn_local(async move {
                info!("MATCH {}", check);
                let json_data = serde_json::json!({
                    "rotor": [
                        slow.as_ref().unwrap_or(&CipherRotor::default()),
                        medium.as_ref().unwrap_or(&CipherRotor::default()),
                        fast.as_ref().unwrap_or(&CipherRotor::default())],
                    "plain": plain,
                    "cryptic": cryptic,
                    "reflector": refl.as_ref().unwrap_or(&Reflector::default()) });
                    //info!("Json {:?}", json_data);
//    let _crep = common::Ciphertext { rotor: [1].to_vec(), plain: "cc".to_string(), cryptic: "aa".to_string(), reflector: "".to_string() };

//                let full_conf.set(
                    let plop: common::Ciphertext = common::Ciphertext{
                        rotor:
                            vec![
                                slow.as_ref().unwrap_or(&CipherRotor::default()).clone(),
                                medium.as_ref().unwrap_or(&CipherRotor::default()).clone(),
                                fast.as_ref().unwrap_or(&CipherRotor::default()).clone()
                        ],
                     plain: plain,
                     cryptic: cryptic,
                     reflector: refl.as_ref().unwrap_or(&Reflector::default()).clone()};
                     full_conf.set(Some(plop));
  //              });
                let response = Request::post(check.as_str())
                    .header("Content-Type", "application/json")
                    .body(json_data.to_string()).expect("DRAMA")
                    .send().await;

                match response {
                    Ok(resp) if resp.ok() => {
                         let crep_state: common::Response = resp.json().await.unwrap();
                         message.set(format!("{:?}", resp).into());
                         full_logs.set(Some(crep_state.debug_logs));
                         //full_conf.set(Some(json_data));
                         resp_state.set((crep_state.plain, crep_state.cryptic))

                    }
                    _ => message.set(format!("Failed {:?}", response).into()),
                }
            });
        })
    };

    html! {
        <div>
            <span class="watermark">{ "Enigma..." }</span>
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
                    <button onclick={get_responses.reform(|_| "scrumble")} >
                        { "Encrypt" }
                    </button>
                    <button onclick={get_responses.reform(|_| "unscrumble")} >
                        { "Decrypt" }
                    </button>

                </div>

                </div>
                <br />

                <div id="logs">
                if let Some(conf) = &*full_conf {
                    <ConfigurationDisplay my_conf={ conf.clone() } />
                }


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
