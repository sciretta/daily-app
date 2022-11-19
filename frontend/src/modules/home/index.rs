use crate::{modules::shared::components::Checkbox, Task};
use gloo::{self, console::log};
use reqwasm::http::Request;
use serde_json::json;
use yew::prelude::*;
use yew_hooks::prelude::*;

struct DayCard {
    day: String,
    tasks: Vec<Task>,
}

impl DayCard {
    fn view(
        &self,
        get_tasks: fn(tasks: UseStateHandle<Vec<Task>>),
        tasks_state: UseStateHandle<Vec<Task>>,
    ) -> Html {
        let tasks = self.tasks.clone();

        html! {
            <div class="mdc-card p-6 mb-1 w-full">
                <div class="text-xl font-bold" >{self.day.clone()}</div>

                  { tasks.into_iter().map(|task|{
                    let is_open = use_state(|| false);
                    let options_menu_class = if *is_open {"mdc-menu absolute right-0 z-10"} else{"mdc-menu mdc-menu-surface"};

                    let node = use_node_ref();

                    use_click_away(node.clone(), {
                        let is_open = is_open.clone();
                        move |_: Event| {
                            if !(*is_open) {return;}
                            is_open.set(false);
                        }
                    });

                    let edit_callback =  {
                        let task = task.clone();
                        Callback::from(move |_| {
                            log!("edit task",serde_json::to_string_pretty(&task.id).unwrap());
                        })
                    };

                    let delete_callback =  {
                        let task = task.clone();
                        let is_open = is_open.clone();
                        let tasks_state = tasks_state.clone();
                        Callback::from(move |_| {
                            if !(*is_open) {return;}
                            let data = json!({
                                "id": task.id.parse::<u8>().unwrap()
                              });
                              wasm_bindgen_futures::spawn_local(async move {
                                  Request::post("http://localhost:8000/api/delete-task")
                                      // .header("Content-Type", "application/json")
                                      .body(data.to_string())
                                      .send()
                                      .await
                                      .unwrap()
                                      .json::<Task>()
                                      .await
                                      .unwrap();
                              });
                              get_tasks(tasks_state.clone());
                        })
                    };


                    html!{
                        <div class="flex justify-between">
                        <div class="flex center items-center">
                        <Checkbox on_check={Callback::from(move |val|log!("test2",val,task.id.clone()))} />

                        <span>{task.name}</span></div>

                        <div>

                        if !(*is_open) {
                            <div class="mdc-button mdc-button--touch" onclick={ Callback::from(move |_| {
                                if *is_open {return;}
                                is_open.set(!*is_open)
                            })}>
                                <i class="material-icons mdc-list-item__graphic text-slate-500" aria-hidden="true">{"more_horiz"}</i>
                            </div>
                        }

                        <div class={options_menu_class} ref={node}>

                            <ul class="mdc-list mdc-card" role="menu" aria-hidden="true" aria-orientation="vertical" tabindex="-1">
                                <li class="mdc-list-item p-4" role="menuitem" onclick={edit_callback}>
                                <span class="mdc-list-item__ripple"></span>
                                <i class="material-icons mdc-list-item__graphic text-slate-500" aria-hidden="true">{"edit"}</i>
                                <span class="mdc-list-item__text">{"Edit"}</span>
                                </li>
                                <li class="mdc-list-item p-4" role="menuitem" onclick={delete_callback}>
                                <span class="mdc-list-item__ripple"></span>
                                <i class="material-icons mdc-list-item__graphic text-slate-500" aria-hidden="true">{"delete"}</i>
                                <span class="mdc-list-item__text">{"Delete"}</span>
                                </li>
                            </ul>
                            </div>
                            </div>

                        </div>
                        } }).collect::<Html>() }
            </div>
        }
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let tasks = use_state(|| vec![]);

    fn get_tasks(tasks: UseStateHandle<Vec<Task>>) {
        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::get("http://localhost:8000/api/get-tasks")
                .send()
                .await
                .unwrap()
                .json::<Vec<Task>>()
                .await
                .unwrap();

            // log!(serde_json::to_string_pretty(&response).unwrap()); // DELETE THIS
            tasks.set(response)
        });
    }

    {
        let tasks = tasks.clone();
        use_effect_with_deps(
            move |_| {
                get_tasks(tasks);
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
            { sorted_tasks.map(|card|card.view(get_tasks,tasks.clone())).collect::<Html>() }
        </div>
    }
}
