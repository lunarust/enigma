use yew::prelude::*;
use log::info;
use common::*;

#[derive(Properties, PartialEq)]
pub struct ShowListProps {
    //pub name: AttrValue,
    pub on_click: Callback<CipherRotor>,
}

#[function_component(RotorsDisplay)]
pub fn rotor_display( ShowListProps { on_click }: &ShowListProps ) -> HtmlResult {
    let selected_rotor_id = use_state(|| None::<i32>);

    let rotors_all:  Vec<CipherRotor> = common::rotor_setup();

    let on_select = |rotor: &CipherRotor| {
        let on_click = on_click.clone();
        let rotor = rotor.clone();
        let selected_rotor_id = selected_rotor_id.clone();
        Callback::from(move |_| {
            selected_rotor_id.set(Some(rotor.id));
            on_click.emit(rotor.clone())
        })
    };

    //info!("SLOW {:?}", selected_rotor_id);

    Ok(
        html!{
            <ul>
            for cr in rotors_all {

                if cr.id == selected_rotor_id.unwrap_or(0) {
                    <li key={cr.id}
                        class="selected"
                        onclick={on_select(&cr)}
                    >
                    { format!("{} - {} Notch: {:?}", &cr.model, &cr.name, &cr.notch) }
                    </li>
                }
                else {
                    <li key={cr.id}
                    onclick={on_select(&cr)}
                    >
                    { format!("{} - {} Notch: {:?}", &cr.model, &cr.name, &cr.notch) }
                    </li>

                }
            }
            </ul>
        }
    )
}
