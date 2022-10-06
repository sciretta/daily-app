#[macro_use]
extern crate rocket;
use chrono::Weekday;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{
    fs::{relative, FileServer},
    serde::Deserialize,
};
use server::TaskType;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// first we need to serve the static files and then we can create the api

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct TaskInput {
    task_type: TaskType,
    date: Option<String>,
    week_days: Option<Vec<Weekday>>,
    time: Option<u8>,
    done: bool,
    name: String,
}

#[post("/new-task", data = "<data>")]
fn new_task(data: Json<TaskInput>) -> status::Accepted<String> {
    println!("{:?}", data);
    let id = 1;
    status::Accepted(Some(format!("id: '{}'", id)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("dist")))
        .mount("/api", routes![index, new_task])
}
