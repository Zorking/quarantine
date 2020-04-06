#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
// #[macro_use] extern crate log;
// #[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod database;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use diesel::SqliteConnection;

use crate::database::{Task, Todo};

// The type to represent the ID of a message.
type ID = usize;

type MessageMap = Mutex<HashMap<ID, String>>;


#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String
}

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

#[derive(Debug, Serialize)]
struct Context{ tasks: Vec<Task> }

#[get("/")]
fn index(conn: DbConn) -> Json< Context >  {
    Json(Context{tasks: Task::all(&conn)})
}


#[get("/<id>", format = "json")]
fn get(id: i32, conn: DbConn) -> Json<Task> {
    Json(Task::get_by_id(id, &conn))
}

#[post("/", format = "json", data = "<message>")]
fn new(message: Json<Todo>, conn: DbConn) -> JsonValue {
    Task::insert(message.0, &conn);
    json!({ "status": "ok" })
}

#[put("/<id>", format = "json", data = "<message>")]
fn update(id: i32, message: Json<Todo>, conn: DbConn) -> JsonValue {
    Task::update(id,message.0, &conn);
    json!({ "status": "ok" })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/message", routes![new, update, get, index])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}
