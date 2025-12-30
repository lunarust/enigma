use yew::prelude::*;
use log::info;
use common::*;

#[derive(Properties, PartialEq)]
pub struct ShowListProps {
    pub on_click: Callback<Reflector>,
}

#[function_component(ReflectorDisplay)]
pub fn reflector_display( ShowListProps { on_click }: &ShowListProps ) -> HtmlResult {
    let selected_reflector_id = use_state(|| None::<i32>);

    let reflector_all:  Vec<Reflector> = common::reflector_setup();

    let on_select = |reflector: &Reflector| {
        let on_click = on_click.clone();
        let reflector = reflector.clone();
        let selected_reflector_id = selected_reflector_id.clone();
        Callback::from(move |_| {
            selected_reflector_id.set(Some(reflector.id));
            on_click.emit(reflector.clone())
        })
    };

    //info!("SLOW {:?}", selected_rotor_id);

    Ok(
        html!{
            <ul>
            for cr in reflector_all {

                if cr.id == selected_reflector_id.unwrap_or(4) {
                    <li key={cr.id}
                        class="selected"
                        onclick={on_select(&cr)}
                    >
                    { &cr.name }
                    </li>
                }
                else {
                    <li key={cr.id}
                    onclick={on_select(&cr)}
                    >
                    { &cr.name }
                    </li>

                }
            }
            </ul>
        }
    )
}
