pub mod fake;
pub mod modules;
mod router;

use crate::router::{switch, Route};

use chrono::{Datelike, Local, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::fmt;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum TaskType {
    TODO,
    HABIT,
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    id: String,
    task_type: TaskType,
    date: Option<String>,
    week_days: Option<Vec<Weekday>>,
    time: Option<u8>,
    name: String,
}

impl Task {
    fn new_habit(id: String, name: String, week_days: Vec<Weekday>) -> Task {
        Task {
            id,
            task_type: TaskType::HABIT,
            date: Option::None,
            week_days: Some(week_days),
            time: Option::None,
            name,
        }
    }

    fn new_todo(id: String, name: String, date: Option<String>) -> Task {
        Task {
            id,
            task_type: TaskType::TODO,
            date,
            week_days: Option::None,
            time: Option::None,
            name,
        }
    }

    fn sort_tasks_by_day(tasks: &Vec<Task>) -> Vec<(String, Vec<Task>)> {
        let mut tasks_sorted: Vec<(String, Vec<Task>)> = Vec::new();
        let mut days: Vec<String> = Vec::new();

        // add days in a vector of strings
        let mut was_today_added = false;
        for task in tasks {
            if task.task_type != TaskType::TODO {
                if !was_today_added {
                    days.push(Local::today().naive_local().to_string());
                }
                was_today_added = true;
                continue;
            }
            match &task.date {
                Some(date) => {
                    if !days.contains(&date) {
                        days.push(date.clone());
                    }
                }
                None => {
                    if !was_today_added {
                        days.push(Local::today().naive_local().to_string());
                    }
                    was_today_added = true;
                }
            }
        }

        // sort the days
        let mut days_sorted = days
            .clone()
            .iter()
            .map(|day| NaiveDate::parse_from_str(day, "%Y-%m-%d").unwrap())
            .collect::<Vec<NaiveDate>>();

        days_sorted.sort_by(|a, b| a.cmp(&b));

        for day in days_sorted {
            let mut tasks_in_this_day: Vec<Task> = Vec::new();
            for task in tasks {
                if task.date.is_some() && (task.date.clone().unwrap() == day.to_string()) {
                    tasks_in_this_day.push(task.clone());
                    continue;
                }

                let is_today = Local::today().naive_local() == day;
                if task.task_type == TaskType::TODO && task.date.is_none() && is_today {
                    tasks_in_this_day.push(task.clone());
                    continue;
                }

                let week_day_of_the_day = day.weekday();
                if task.task_type == TaskType::HABIT
                    && task.date.is_none()
                    && task.week_days.is_some()
                {
                    let includes_the_day = task
                        .week_days
                        .clone()
                        .unwrap()
                        .contains(&week_day_of_the_day);
                    if includes_the_day {
                        tasks_in_this_day.push(task.clone());
                        continue;
                    }
                }
            }
            tasks_sorted.push((day.to_string(), tasks_in_this_day));
        }

        tasks_sorted
    }
}
