use yew::prelude::*;
use crate::modules::task::components::{new_task::NewTask, edit_task::TaskEdit};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: String
}


#[function_component(Task)]
pub fn task(props: &Props) -> Html {

    if props.id.to_lowercase() == "new" {
        html!{
            <NewTask />
        }
    }else {
        html!{
           <TaskEdit id={props.id.clone()}/>
        }
    }
}