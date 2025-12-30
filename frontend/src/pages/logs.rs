use yew::prelude::*;
use common::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub my_logs: Vec<DebugLogs>,
}

#[function_component(LogsDisplay)]
pub fn logs_display( Props { my_logs }: &Props ) -> HtmlResult {
    Ok(
        html!{
            for fl in my_logs {
                <span class="logs_id">{" "}{fl.idx}{" "}</span>
                <span class="logs_offset">
                for floff in fl.offset.clone() {
                    {format!("{}", floff)}    
                }
                </span>

                for flpass in fl.pass.clone() {
                    <span class="logs_pass">{ " " }{flpass}{ " " }</span>
                }
                <br />
            }
        }
    )
}
