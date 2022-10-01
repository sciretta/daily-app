use yew::prelude::*;
use crate::{Task, modules::shared::components::Checkbox, fake::generate_fake_tasks};
use gloo::{self, console::log};

struct DayCard {
    day:String,
    tasks: Vec<Task>
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
    let sorted_tasks = Task::sort_tasks_by_day(&generate_fake_tasks(10)).into_iter().map(|group|DayCard{day:group.0.clone(),tasks:group.1.clone()});

    html!{
        <div class="view-container">
            { sorted_tasks.map(|card|card.view()).collect::<Html>() }
        </div>
    }
}