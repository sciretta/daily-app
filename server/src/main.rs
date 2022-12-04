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
use server::{verify_and_parse_input_record, ManageDatabase, Task, TaskType};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SelectedTask {
    id: u8,
}

#[get("/get-tasks")]
fn get_tasks() -> status::Custom<RawJson<String>> {
    let lines = ManageDatabase::read_data();
    let mut tasks: Vec<Task> = vec![];

    for line in lines.clone() {
        if line.contains("id::name::type::date::week,days") {
            continue;
        }
        tasks.push(verify_and_parse_input_record(line));
    }

    let serialized_tasks = serde_json::to_string(&tasks).unwrap();

    status::Custom(Status::Accepted, content::RawJson(serialized_tasks))
}

#[post("/get-task", data = "<data>")]
fn get_task(data: Json<SelectedTask>) -> status::Custom<RawJson<String>> {
    println!("{:?}", data);
    let lines = ManageDatabase::read_data();
    let mut tasks: Vec<Task> = vec![];

    for line in lines.clone() {
        if line.contains("id::name::type::date::week,days") {
            continue;
        }
        tasks.push(verify_and_parse_input_record(line));
    }

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
    let lines = ManageDatabase::read_data();
    let mut tasks: Vec<Task> = vec![];

    println!("{}", data.id.unwrap());

    for line in lines.clone() {
        if line.contains("id::name::type::date::week,days") {
            continue;
        }
        let current_task = verify_and_parse_input_record(line);
        if current_task.name == data.name && current_task.id != data.id.unwrap().to_string() {
            panic!("This name is already in use")
        }
        tasks.push(current_task);
    }

    tasks[usize::from(data.id.unwrap() - 1)].name = data.name.clone();
    tasks[usize::from(data.id.unwrap() - 1)].task_type = data.task_type.clone();
    tasks[usize::from(data.id.unwrap() - 1)].date = data.date.clone();
    if data.week_days.is_some() {
        tasks[usize::from(data.id.unwrap() - 1)].week_days = data.week_days.clone();
    }

    let mut lines_updated: Vec<String> = vec![String::from("id::name::type::date::week,days")];

    for current_task in tasks {
        let mut new_line: String = String::from("");

        new_line = format!(
            "{}::{}::{}::{}::{}",
            lines_updated.len(),
            current_task.name,
            match current_task.task_type {
                TaskType::HABIT => "HABIT",
                TaskType::TODO => "TODO",
            },
            match &current_task.date {
                Some(date) => date,
                None => "null",
            },
            match &current_task.week_days {
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
            }
        );
        verify_and_parse_input_record(new_line.clone());
        lines_updated.push(new_line);
    }

    let parsed_data: String = lines_updated.join("\n");

    ManageDatabase::write_data(parsed_data);

    status::Accepted(Some(format!("id: '{}'", data.id.unwrap())))
}

#[post("/new-task", data = "<data>")]
fn new_task(data: Json<TaskInput>) -> status::Accepted<String> {
    let mut lines = ManageDatabase::read_data();

    for line in lines.clone() {
        if line.contains("id::name::type::date::week,days") {
            continue;
        }
        let current_line = verify_and_parse_input_record(line);
        if current_line.name == data.name {
            panic!("This name is already in use")
        }
    }

    let new_line: String = format!(
        "{}::{}::{}::{}::{}",
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
        }
    );

    verify_and_parse_input_record(new_line.clone());

    lines.push(new_line);

    let parsed_data: String = lines.join("\n");

    ManageDatabase::write_data(parsed_data);

    status::Accepted(Some(format!("id: '{}'", lines.len() - 1)))
}

#[post("/delete-task", data = "<data>")]
fn delete_task(data: Json<SelectedTask>) -> status::Accepted<String> {
    println!("{:?}", data);

    let lines = ManageDatabase::read_data();
    let mut db_tasks: Vec<Task> = vec![];

    let mut line_to_delete_exists = false;
    for line in lines.clone() {
        if line.contains("id::name::type::date::week,days") {
            continue;
        }
        let current_line = verify_and_parse_input_record(line);
        if current_line.id == data.id.to_string() {
            line_to_delete_exists = true;
        }
        db_tasks.push(current_line);
    }
    if !line_to_delete_exists {
        panic!("This task does not exist");
    }

    let mut lines_updated: Vec<String> = vec![String::from("id::name::type::date::week,days")];

    for current_task in db_tasks {
        let mut new_line: String = String::from("");

        if current_task.id.parse::<u8>().unwrap() != data.id {
            new_line = format!(
                "{}::{}::{}::{}::{}",
                lines_updated.len(),
                current_task.name,
                match current_task.task_type {
                    TaskType::HABIT => "HABIT",
                    TaskType::TODO => "TODO",
                },
                match &current_task.date {
                    Some(date) => date,
                    None => "null",
                },
                match &current_task.week_days {
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
                }
            );
        } else {
            continue;
        }
        verify_and_parse_input_record(new_line.clone());
        lines_updated.push(new_line);
    }

    let parsed_data: String = lines_updated.join("\n");

    ManageDatabase::write_data(parsed_data);

    status::Accepted(Some(format!("Task deleted: '{}'", data.id)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", FileServer::from(relative!("dist")))
        .mount(
            "/api",
            routes![get_tasks, new_task, delete_task, get_task, update_task],
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
