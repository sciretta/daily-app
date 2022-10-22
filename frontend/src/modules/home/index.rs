use crate::{modules::shared::components::Checkbox, Task};
use gloo::{self, console::log};
use reqwasm::http::Request;
use serde_json;
use yew::prelude::*;

struct DayCard {
    day: String,
    tasks: Vec<Task>,
}

impl DayCard {
    fn view(&self) -> Html {
        let tasks = self.tasks.clone();

        html! {
            <div class="mdc-card p-6 mb-1 w-full">
                <div class="text-xl font-bold" >{self.day.clone()}</div>

                  { tasks.into_iter().map(|task|{
                    html!{
                    <div class="flex items-center ">
                    <Checkbox on_check={Callback::from(move |val|log!("test2",val,task.id.clone()))} />

                    <span>{task.name}</span></div>
                 } }).collect::<Html>() }
            </div>
        }
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let tasks = use_state(|| vec![]);

    {
        let tasks = tasks.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("http://localhost:8000/api/get_tasks")
                        .send()
                        .await
                        .unwrap()
                        .json::<Vec<Task>>()
                        .await
                        .unwrap();

                    log!(serde_json::to_string_pretty(&response).unwrap()); // DELETE THIS
                    tasks.set(response)
                });
                || ()
            },
            (),
        );
    }

    let sorted_tasks = Task::sort_tasks_by_day(&tasks)
        .into_iter()
        .map(|group| DayCard {
            day: group.0.clone(),
            tasks: group.1.clone(),
        });

    html! {
        <div class="view-container">
            { sorted_tasks.map(|card|card.view()).collect::<Html>() }
        </div>
    }
}
