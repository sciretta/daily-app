#[macro_use]
extern crate rocket;
use chrono::Weekday;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::http::Status;
use rocket::response::content::RawJson;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use rocket::{
    fs::{relative, FileServer},
    serde::Deserialize,
};
use rocket::{Request, Response};
use server::{DbRecord, ManageDatabase, Task, TaskType};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SelectedTask {
    id: u8,
}

#[get("/get-tasks")]
fn get_tasks() -> status::Custom<RawJson<String>> {
    let tasks = Task::read_data();

    let serialized_tasks = serde_json::to_string(&tasks).unwrap();

    status::Custom(Status::Accepted, content::RawJson(serialized_tasks))
}

#[post("/get-task", data = "<data>")]
fn get_task(data: Json<SelectedTask>) -> status::Custom<RawJson<String>> {
    println!("{:?}", data);
    let tasks = Task::read_data();

    let serialized_task = serde_json::to_string(&tasks[usize::from(data.id - 1)]).unwrap();

    status::Custom(Status::Accepted, content::RawJson(serialized_task))
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TaskInput {
    task_type: TaskType,
    date: Option<String>,
    week_days: Option<Vec<Weekday>>,
    // time: Option<u8>,
    id: Option<u8>,
    name: String,
}

#[post("/update-task", data = "<data>")]
fn update_task(data: Json<TaskInput>) -> status::Accepted<String> {
    let mut tasks = Task::read_data();

    println!("{}", data.id.unwrap());

    for task in tasks.clone() {
        if task.name == data.name && task.id != data.id.unwrap().to_string() {
            panic!("This name is already in use")
        }
    }

    tasks[usize::from(data.id.unwrap() - 1)].name = data.name.clone();
    tasks[usize::from(data.id.unwrap() - 1)].task_type = data.task_type.clone();
    tasks[usize::from(data.id.unwrap() - 1)].date = data.date.clone();
    if data.week_days.is_some() {
        tasks[usize::from(data.id.unwrap() - 1)].week_days = data.week_days.clone();
    }

    let mut lines_updated: Vec<String> = vec![String::from("id::name::type::date::week,days")];

    for current_task in tasks {
        let new_task: Task = Task {
            name: current_task.name,
            id: lines_updated.len().to_string(),
            date: current_task.date,
            task_type: current_task.task_type,
            week_days: current_task.week_days,
        };
        let new_line = new_task.to_string();

        Task::string_to_record(new_line.clone());
        lines_updated.push(new_line);
    }

    let parsed_data: String = lines_updated.join("\n");

    Task::write_data(parsed_data);

    status::Accepted(Some(format!("id: '{}'", data.id.unwrap())))
}

#[post("/new-task", data = "<data>")]
fn new_task(data: Json<TaskInput>) -> status::Accepted<String> {
    let tasks = Task::read_data();

    for task in tasks.clone() {
        if task.name == data.name {
            panic!("This name is already in use")
        }
    }

    let new_task: Task = Task {
        name: data.name.clone(),
        id: (tasks.len() + 1).to_string(),
        date: data.date.clone(),
        task_type: data.task_type.clone(),
        week_days: data.week_days.clone(),
    };

    Task::new_record(new_task);

    status::Accepted(Some(format!("id: '{}'", tasks.len() + 1)))
}

#[post("/delete-task", data = "<data>")]
fn delete_task(data: Json<SelectedTask>) -> status::Accepted<String> {
    println!("{:?}", data);

    Task::remove_record(data.id.to_string());

    status::Accepted(Some(format!("Task deleted: '{}'", data.id)))
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DoneInput {
    date: String,
    time: u8,
    id: u8,
}

// #[post("/done-task", data = "<data>")]
// fn done_and_undone_task(data: Json<DoneInput>) -> status::Accepted<String> {
//     let stats = Stat::read_data();

//     for record in stats.iter() {
//         println!("{}", record.id)
//     }

//     println!("{}", data.id);

//     status::Accepted(Some(format!("id: '{}'", data.id)))
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", FileServer::from(relative!("dist")))
        .mount(
            "/api",
            routes![
                get_tasks,
                new_task,
                delete_task,
                get_task,
                update_task,
                // done_and_undone_task
            ],
        )
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
