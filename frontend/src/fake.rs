use chrono::{Local, Duration, Weekday::{
    Fri,
    Mon,
    Sat,
    Sun,
    Wed,
    Tue,
    Thu,
}, Datelike};
use rand::prelude::*;
use crate::{Task};

pub fn generate_fake_tasks (amount:usize) -> Vec<Task> {
    let mut todo_tasks: Vec<Task> = vec![
        Task::new_todo(String::from("1"),String::from("todo task 1"), Option::None),
    ];

    let mut habit_tasks: Vec<Task> = vec![
        Task::new_habit(String::from((amount/2).to_string()),String::from(format!("habit task {}",(amount/2))), vec![
            Mon,
            Tue,
            // Wed,
            // Thu,
            // Fri,
            Sat,
            Sun
            ]),
    ];

    while todo_tasks.len() < (amount/2) {

        let mut rng = rand::thread_rng();
        let days_to_push: u8 = rng.gen_range(1..20); 

        let new_date = Local::now().date_naive() + Duration::days(days_to_push.into());

        todo_tasks.push( 
            Task::new_todo((todo_tasks.len() +1).to_string(),format!("todo task {}",todo_tasks.len() +1), Some(new_date.to_string()))
        );
        
    }

    while habit_tasks.len() < (amount/2) {

        let mut rng = rand::thread_rng();

        let days_to_push_one: u8 = rng.gen_range(1..20); 

        let week_day_one = (Local::now().date_naive() + Duration::days(days_to_push_one.into())).weekday();
        let week_day_two = (Local::now().date_naive() + Duration::days((days_to_push_one + 1).into())).weekday();
        let week_day_three = (Local::now().date_naive() + Duration::days((days_to_push_one +2).into()) ).weekday();


        habit_tasks.push( 
            Task::new_habit(String::from((amount/2 + habit_tasks.len() + 1).to_string()),String::from(format!("habit task {}",(amount/2 + habit_tasks.len() + 1))), vec![week_day_one, week_day_two, week_day_three])
        );
        
    }

    todo_tasks.append(&mut habit_tasks);
    todo_tasks
}