use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

use common::*;
use crate::pages::rotors::RotorsDisplay;
use crate::pages::reflector::ReflectorDisplay;
use crate::pages::logs::LogsDisplay;
use crate::pages::configuration::ConfigurationDisplay;
//use log::info;

//#[derive(Clone, Copy, PartialEq)]
//enum RotorPos { Slow, Medium, Fast }

#[function_component]
pub fn Home() -> Html {
    let message = use_state(|| "".to_string());
    let full_logs = use_state(|| None);
    let full_conf = use_state(|| None);
    let plugboard_state = use_state(|| "".to_string());
    let resp_state = use_state(|| (
        "".to_string(),
        "".to_string()
    ));

    let selected_slow_rotor = use_state(|| None);
    let selected_medium_rotor = use_state(|| None);
    let selected_fast_rotor = use_state(|| None);
    let rotor_start_state = use_state(|| ("A".to_string(), "A".to_string(), "A".to_string()));
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
    let start_state = rotor_start_state.clone();
    let refl = selected_reflector.clone();
    let full_logs = full_logs.clone();
    let full_conf = full_conf.clone();


    let get_responses = {
        let plugboard_state = plugboard_state.clone();
        let resp_state = resp_state.clone();
        let message = message.clone();
        let full_logs = full_logs.clone();
        let full_conf = full_conf.clone();

        Callback::from(move |direction: &str| {
            let check = format!("http://localhost:9000/{}", direction);

            let refl = refl.clone();
            let slow = slow.clone();
            let fast = fast.clone();
            let medium = medium.clone();
            let start_state = start_state.clone();

            let message = message.clone();
            let (plain, cryptic) = (*resp_state).clone();
            let plugboard = plugboard_state.clone();
            let resp_state = resp_state.clone();
            let full_logs = full_logs.clone();
            let full_conf = full_conf.clone();

            spawn_local(async move {
                let plop: common::Ciphertext = common::Ciphertext{
                    rotor:
                        vec![
                            slow.as_ref().unwrap_or(&CipherRotor::default()).clone(),
                            medium.as_ref().unwrap_or(&CipherRotor::default()).clone(),
                            fast.as_ref().unwrap_or(&CipherRotor::default()).clone()
                    ],
                     plain: plain,
                     cryptic: cryptic,
                     plugboard: plugboard.to_string(),
                     reflector: refl.as_ref().unwrap_or(&Reflector::default()).clone(),
                     start_position: vec![start_state.0.clone(), start_state.1.clone(), start_state.2.clone()]

                };
                 let json_data = serde_json::to_string(&plop).expect("Drama");
                 full_conf.set(Some(plop));



                let response = Request::post(check.as_str())
                    .header("Content-Type", "application/json")
                    .body(json_data.to_string()).expect("DRAMA")
                    .send().await;

                match response {
                    Ok(resp) if resp.ok() => {
                         let crep_state: common::Response = resp.json().await.unwrap();
                         message.set(format!("{:?}", resp).into());
                         full_logs.set(Some(crep_state.debug_logs));

                         resp_state.set((crep_state.plain, crep_state.cryptic))

                    }
                    _ => message.set(format!("Failed {:?}", response).into()),
                }
            });
        })
    };

    html! {
        <div id="main">
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
            {"Start position:   "}
            <input value={rotor_start_state.0.clone()} //{oninput}
                oninput={Callback::from({
                let rotor_start_state = rotor_start_state.clone();
                move |e: InputEvent| {
                    let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                    rotor_start_state.set((input.value(), rotor_start_state.1.clone(), rotor_start_state.2.clone()));
                }
                })}
                maxlength="1"
                class="start_letter"
                key="fast_start" />
            <hr />
            <h4>{"MEDIUM: Middle position"}</h4>
            <RotorsDisplay
                on_click={
                    let cb = on_rotor_select.clone();
                    cb.reform(|rotor| ("medium".to_string(), rotor))
                }
             />
            {"Start position:   "}
                <input value={rotor_start_state.1.clone()} //{oninput}
                oninput={Callback::from({
                let rotor_start_state = rotor_start_state.clone();
                move |e: InputEvent| {
                    let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                    rotor_start_state.set((rotor_start_state.0.clone(), input.value(), rotor_start_state.2.clone()));
                }
                })}
                class="start_letter"
                maxlength="1"
                key="medium_start" />

             <hr />
             <h4>{"SLOW: Left position"}</h4>
             <RotorsDisplay
                on_click={
                    let cb = on_rotor_select.clone();
                    cb.reform(|rotor| ("fast".to_string(), rotor))
                }
             />
            {"Start position:   "}
                <input value={rotor_start_state.2.clone()}
                oninput={Callback::from({
                let rotor_start_state = rotor_start_state.clone();
                move |e: InputEvent| {
                    let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                    rotor_start_state.set((rotor_start_state.0.clone(), rotor_start_state.1.clone(), input.value()));
                }
                })}
                maxlength="1"
                class="start_letter"
                key="slow_start" />
             <hr />
             <h3>{"Reflector:"}</h3>
             <ReflectorDisplay
                on_click={on_reflector_select}
             />
             <hr />
             <h3>{"Plugboard:     "}
                <input //value={plugboard_state.clone()}
                oninput={Callback::from({
                let plugboard_state = plugboard_state.clone();
                move |e: InputEvent| {
                    let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                    plugboard_state.set(input.value());
                }
                })}
                maxlength="15"
                class="plugboard"
                />
                { "  " }
                </h3>
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

            </div>
                <div id="logs">
                if let Some(conf) = &*full_conf {
                    <ConfigurationDisplay my_conf={ conf.clone() } />
                }

                if let Some(full_logs) = &*full_logs {
                    <LogsDisplay my_logs={ full_logs.clone() } />
                }
                </div>
            if !message.is_empty() {
                <p class="footer">{ &*message }</p>
            }
        <span class="watermark">{ "enigma..." }</span>

        </div>
    }
}
