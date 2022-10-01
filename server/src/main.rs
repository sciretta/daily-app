#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// first we need to serve the static files and then we can create the api

#[post("/new-task")]
fn new_task() -> &'static str {
    "new task"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, new_task])
}
