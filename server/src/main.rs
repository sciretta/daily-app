#[macro_use]
extern crate rocket;
use chrono::Weekday;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{
    fs::{relative, FileServer},
    serde::Deserialize,
};
use server::{verify_and_parse_input_record, ManageDatabase, TaskType};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// first we need to serve the static files and then we can create the api

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TaskInput {
    task_type: TaskType,
    date: Option<String>,
    week_days: Option<Vec<Weekday>>,
    // time: Option<u8>,
    done: bool,
    name: String,
}

#[post("/new-task", data = "<data>")]
fn new_task(data: Json<TaskInput>) -> status::Accepted<String> {
    let mut lines = ManageDatabase::read_data();

    for line in lines.clone() {
        if line.contains("id::name::type::date::week,days::done") {
            continue;
        }
        let current_line = verify_and_parse_input_record(line);
        if current_line.name == data.name {
            panic!("This name is already in use")
        }
    }

    let new_line: String = format!(
        "{}::{}::{}::{}::{}::{}",
        lines.len(),
        data.name,
        match data.task_type {
            TaskType::HABIT => "HABIT",
            TaskType::TODO => "TODO",
        },
        match &data.date {
            Some(date) => date,
            None => "null",
        },
        match &data.week_days {
            Some(week_days) => week_days
                .iter()
                .map(|day| match day {
                    Weekday::Mon => "MON".to_string(),
                    Weekday::Tue => "TUE".to_string(),
                    Weekday::Wed => "WED".to_string(),
                    Weekday::Thu => "THU".to_string(),
                    Weekday::Fri => "FRI".to_string(),
                    Weekday::Sat => "SAT".to_string(),
                    Weekday::Sun => "SUN".to_string(),
                })
                .collect::<Vec<String>>()
                .join(","),
            None => "null".to_string(),
        },
        data.done
    );

    verify_and_parse_input_record(new_line.clone());

    lines.push(new_line);

    let parsed_data: String = lines.join("\n");

    print!("{:?}", parsed_data);

    ManageDatabase::write_data(parsed_data);

    status::Accepted(Some(format!("id: '{}'", 1)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("dist")))
        .mount("/api", routes![index, new_task])
}
