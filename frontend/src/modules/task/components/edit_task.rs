use crate::{
    modules::shared::components::{Checkbox, Switch},
    router::Route,
    Task, TaskType,
};
use chrono::Weekday::{self, Fri, Mon, Sat, Sun, Thu, Tue, Wed};
use gloo::{self, console::log};
use reqwasm::http::Request;
use serde_json::{self, json};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

#[function_component(TaskEdit)]
pub fn edit_task(props: &Props) -> Html {
    let task_name = use_state(|| String::from(""));
    let task_type = use_state(|| TaskType::HABIT);
    let have_date = use_state(|| false);
    let task_date = use_state(|| String::from(""));

    let selected_week_days = use_state(|| vec![Mon]);
    let can_save = use_state(|| false);

    let history = use_history().unwrap();

    {
        let id = props.id.clone().parse::<u8>().unwrap();
        let task_name = task_name.clone();
        let task_type = task_type.clone();
        let have_date = have_date.clone();
        let task_date = task_date.clone();
        let selected_week_days = selected_week_days.clone();
        use_effect_with_deps(
            move |_| {
                let data = json!({ "id": id });
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::post("http://localhost:8000/api/get-task")
                        .body(data.to_string())
                        .send()
                        .await
                        .unwrap()
                        .json::<Task>()
                        .await
                        .unwrap();

                    // log!(serde_json::to_string_pretty(&response.week_days.unwrap()).unwrap());

                    task_name.set(response.name);
                    task_type.set(response.task_type);
                    have_date.set(response.date.is_some());
                    task_date.set(match response.date {
                        Some(value) => value,
                        None => "".to_string(),
                    });
                    selected_week_days.set(match response.week_days {
                        Some(value) => value,
                        None => vec![Mon],
                    });
                });
                || ()
            },
            (),
        );
    }

    let on_change_name = {
        let task_name = task_name.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            task_name.set(value);
        })
    };

    {
        let can_save_clone = can_save.clone();
        let task_name_clone = task_name.clone();
        let task_type_clone = task_type.clone();
        let have_date_clone = have_date.clone();
        let task_date_clone = task_date.clone();
        use_effect_with_deps(
            move |_| {
                if (*task_name_clone).len() < 3 {
                    can_save_clone.set(false);
                } else if *task_type_clone == TaskType::TODO
                    && *have_date_clone
                    && (*task_date_clone).len() == 0
                {
                    can_save_clone.set(false);
                } else {
                    can_save_clone.set(true);
                }
                || ()
            },
            (
                task_name.clone(),
                task_type.clone(),
                task_date.clone(),
                have_date.clone(),
                selected_week_days.clone(),
            ),
        );
    }

    let save_data = {
        let task_name = task_name.clone();
        let task_type = task_type.clone();
        let task_date = task_date.clone();
        let have_date = have_date.clone();
        let selected_week_days = selected_week_days.clone();
        let id = props.id.clone().parse::<u8>().unwrap();

        Callback::from(move |_: MouseEvent| {
            let data = json!({
              "task_type": task_type.to_string(),
              "date": if task_date.len() == 0 || !(*have_date) {None}else{Some((*task_date).clone())},
              "week_days": if (*task_type) == TaskType::HABIT {Some(selected_week_days.iter().map(|val|val.to_string()).collect::<Vec<String>>())} else {None},
              "done": false,
              "name": *task_name,
              "id":id.clone()
            });
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                Request::post("http://127.0.0.1:8000/api/update-task")
                    // .header("Content-Type", "application/json")
                    .body(data.to_string())
                    .send()
                    .await
                    .unwrap();

                history.push(Route::Home.clone())
            });
        })
    };

    let on_change_date = {
        let task_date = task_date.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            task_date.set(value);
        })
    };

    html! {
           <div class="view-container">
             <div class="text-xl font-bold w-4/5" >{"Task name"}</div>
               <label class="mdc-text-field mdc-text-field--outlined mb-5 w-4/5">
                   <span class="mdc-notched-outline">
                       <span class="mdc-notched-outline__leading"></span>
                       <span class="mdc-notched-outline__trailing"></span>
                   </span>
                   <input type="text" value={(*task_name).clone()} oninput ={on_change_name} class="mdc-text-field__input" aria-labelledby="my-label-id"/>
                   </label>

                   <div class="text-xl font-bold w-4/5" >{"Task type"}</div>
                   <div class="text-l w-4/5 flex mb-5 justify-evenly" >
                       <div  class="flex justify-center items-center">
                           {"Habit"}
                           <Checkbox
                           disabled={*task_type == TaskType::HABIT}
                           checked={*task_type == TaskType::HABIT}
                           on_check={{
                           let task_type = task_type.clone();
                           Callback::from(move |val|if val == true {
                               task_type.set(TaskType::HABIT)
                           } )
                           }} />
                       </div>
                       <div  class="flex justify-center items-center" >
                         {"Todo"}
                         <Checkbox
                           disabled={*task_type == TaskType::TODO}
                           checked={*task_type == TaskType::TODO}
                           on_check={{
                           let task_type = task_type.clone();
                           Callback::from(move |val|if val == true {
                               task_type.set(TaskType::TODO)
                           } )
                         }} />
                       </div>
                   </div>

                   if *task_type == TaskType::HABIT {
                       <>
                           <div class="text-xl font-bold w-4/5" >{"select the week days"}</div>
                           <div class="flex flex-col">
                               <div  class="flex justify-end items-center">
                                   {"Monday"}
                                   <Checkbox
                                       checked={selected_week_days.contains(&Mon)}
                                       on_check={{
                                           let selected_week_days = selected_week_days.clone();

                                           Callback::from(move |_| {
                                               let mut new_value: Vec<Weekday> = (*selected_week_days).clone();
                                               if selected_week_days.len() > 1 && selected_week_days.contains(&Mon) {
                                                   let index = new_value.iter().position(|x| *x == Mon).unwrap();
                                                   new_value.remove(index);
                                               }else if !selected_week_days.contains(&Mon) {
                                                   new_value.push(Mon);

                                               }
                                               selected_week_days.set(new_value.clone());
                                           })
                                       }}
                                   />
                               </div>
                               <div  class="flex justify-end items-center">
                                   {"Tuesday"}
                                   <Checkbox
                                       checked={selected_week_days.contains(&Tue)}
                                       on_check={{
                                           let selected_week_days = selected_week_days.clone();

                                           Callback::from(move |_| {
                                               let mut new_value: Vec<Weekday> = (*selected_week_days).clone();
                                               if selected_week_days.len() > 1 && selected_week_days.contains(&Tue) {
                                                   let index = new_value.iter().position(|x| *x == Tue).unwrap();
                                                   new_value.remove(index);
                                               }else if !selected_week_days.contains(&Tue) {
                                                   new_value.push(Tue);

                                               }
                                               selected_week_days.set(new_value.clone());
                                           })
                                       }}
                                   />
                               </div>
                               <div  class="flex justify-end items-center">
                                   {"Wednesday"}
                                   <Checkbox
                                       checked={selected_week_days.contains(&Wed)}
                                       on_check={{
                                           let selected_week_days = selected_week_days.clone();

                                           Callback::from(move |_| {
                                               let mut new_value: Vec<Weekday> = (*selected_week_days).clone();
                                               if selected_week_days.len() > 1 && selected_week_days.contains(&Wed) {
                                                   let index = new_value.iter().position(|x| *x == Wed).unwrap();
                                                   new_value.remove(index);
                                               }else if !selected_week_days.contains(&Wed){
                                                   new_value.push(Wed);

                                               }
                                               selected_week_days.set(new_value.clone());
                                           })
                                       }}
                                   />
                               </div>
                               <div  class="flex justify-end items-center">
                                   {"Thursday"}
                                   <Checkbox
                                       checked={selected_week_days.contains(&Thu)}
                                       on_check={{
                                           let selected_week_days = selected_week_days.clone();

                                           Callback::from(move |_| {
                                               let mut new_value: Vec<Weekday> = (*selected_week_days).clone();
                                               if selected_week_days.len() > 1 && selected_week_days.contains(&Thu) {
                                                   let index = new_value.iter().position(|x| *x == Thu).unwrap();
                                                   new_value.remove(index);
                                               }else if !selected_week_days.contains(&Thu) {
                                                   new_value.push(Thu);

                                               }
                                               selected_week_days.set(new_value.clone());
                                           })
                                       }}
                                   />
                               </div>
                               <div  class="flex justify-end items-center">
                                   {"Friday"}
                                   <Checkbox
                                       checked={selected_week_days.contains(&Fri)}
                                       on_check={{
                                           let selected_week_days = selected_week_days.clone();

                                           Callback::from(move |_| {
                                               let mut new_value: Vec<Weekday> = (*selected_week_days).clone();
                                               if selected_week_days.len() > 1 && selected_week_days.contains(&Fri) {
                                                   let index = new_value.iter().position(|x| *x == Fri).unwrap();
                                                   new_value.remove(index);
                                               }else if !selected_week_days.contains(&Fri) {
                                                   new_value.push(Fri);

                                               }
                                               selected_week_days.set(new_value.clone());
                                           })
                                       }}
                                   />
                               </div>
                               <div  class="flex justify-end items-center">
                                   {"Saturday"}
                                   <Checkbox
                                       checked={selected_week_days.contains(&Sat)}
                                       on_check={{
                                           let selected_week_days = selected_week_days.clone();

                                           Callback::from(move |_| {
                                               let mut new_value: Vec<Weekday> = (*selected_week_days).clone();
                                               if selected_week_days.len() > 1 && selected_week_days.contains(&Sat) {
                                                   let index = new_value.iter().position(|x| *x == Sat).unwrap();
                                                   new_value.remove(index);
                                               }else if !selected_week_days.contains(&Sat) {
                                                   new_value.push(Sat);

                                               }
                                               selected_week_days.set(new_value.clone());
                                           })
                                       }}
                                   />
                               </div>
                               <div  class="flex justify-end items-center">
                                   {"Sunday"}
                                   <Checkbox
                                       checked={selected_week_days.contains(&Sun)}
                                       on_check={{
                                           let selected_week_days = selected_week_days.clone();

                                           Callback::from(move |_| {
                                               let mut new_value: Vec<Weekday> = (*selected_week_days).clone();
                                               if selected_week_days.len() > 1 && selected_week_days.contains(&Sun)  {
                                                   let index = new_value.iter().position(|x| *x == Sun).unwrap();
                                                   new_value.remove(index);
                                               }else if !selected_week_days.contains(&Sun){
                                                   new_value.push(Sun);

                                               }
                                               selected_week_days.set(new_value.clone());
                                           })
                                       }}
                                   />
                               </div>
                           </div>
                       </>
                   } else {
                       <>
                       <div class="text-xl font-bold w-4/5" >{"Todo date"}</div>
                       <div class="text-l w-4/5 flex mb-5 justify-evenly" >
                           <Switch
                           default_checked={*have_date}
                           on_check={{
                             let have_date = have_date.clone();
                             Callback::from(move |val|have_date.set(val) )
                           }}  />
                       </div>
                       if *have_date {
                           <div class="text-l w-4/5 flex mb-5 justify-center" >
                               <input id="startDate" value={(*task_date).clone()} onchange={on_change_date} class="form-control" type="date" />
                           </div>
                       }
                       </>
                   }


    if *can_save {
               <button class="mdc-button mdc-button--raised w-4/5" onclick={save_data}>
                   <span class="mdc-button__label">{"Save changes"}</span>
               </button>}
           </div>
       }
}
