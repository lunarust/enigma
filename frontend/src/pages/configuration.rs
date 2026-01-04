use yew::prelude::*;
use common::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub my_conf: Ciphertext,
}

#[function_component(ConfigurationDisplay)]
pub fn configuration_display( Props { my_conf }: &Props ) -> HtmlResult {
    Ok(
        html!{
            <span class="configuration">
            <h4>{"Selected configuration:"}</h4>
            for element in my_conf.rotor.clone() {
                 { format!("Model: {}, Name: {}, Definition: {} Notch:{:?}",
                     element.model, element.name, element.definition, element.notch) }<br />
            }
            { format!("Reflector: Name: {}, Definition: {}", my_conf.reflector.name, my_conf.reflector.definition) }<br /><br /><hr />

            </span>
        }
    )
}
