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

#[get("/get-tasks")]
fn get_tasks() -> status::Custom<RawJson<String>> {
    let lines = ManageDatabase::read_data();
    let mut tasks: Vec<Task> = vec![];

    for line in lines.clone() {
        if line.contains("id::name::type::date::week,days::done") {
            continue;
        }
        tasks.push(verify_and_parse_input_record(line));
    }

    let serialized_user = serde_json::to_string(&tasks).unwrap();

    status::Custom(Status::Accepted, content::RawJson(serialized_user))
}

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

    ManageDatabase::write_data(parsed_data);

    status::Accepted(Some(format!("id: '{}'", lines.len() - 1)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", FileServer::from(relative!("dist")))
        .mount("/api", routes![get_tasks, new_task])
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
