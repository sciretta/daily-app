use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: String
}


#[function_component(TaskEdit)]
pub fn task_edit(props: &Props) -> Html {

        html!{
            <div class="view-container">
            {"task view here"} {props.id.clone()}
            </div>
        }
}