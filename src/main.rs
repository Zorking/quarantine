#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod database;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use diesel::SqliteConnection;

use crate::database::{Task};

// The type to represent the ID of a message.
type ID = usize;

// We're going to store all of the messages here. No need for a DB.
type MessageMap = Mutex<HashMap<ID, String>>;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();



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

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/<id>", format = "json", data = "<message>")]
fn new(id: ID, message: Json<Message>, map: State<'_, MessageMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        json!({
            "status": "error",
            "reason": "ID exists. Try put."
        })
    } else {
        hashmap.insert(id, message.0.contents);
        json!({ "status": "ok" })
    }
}

#[put("/<id>", format = "json", data = "<message>")]
fn update(id: ID, message: Json<Message>, map: State<'_, MessageMap>) -> Option<JsonValue> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, message.0.contents);
        Some(json!({ "status": "ok" }))
    } else {
        None
    }
}

#[get("/<id>", format = "json")]
fn get(id: ID, map: State<'_, MessageMap>) -> Option<Json<Message>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| {
        Json(Message {
            id: Some(id),
            contents: contents.clone()
        })
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn run_db_migrations(rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(rocket::fairing::AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount("/message", routes![new, update, get, index])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}
