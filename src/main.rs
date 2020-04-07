#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod database;

use rocket_contrib::json::{Json, JsonValue};

use diesel::SqliteConnection;

use crate::database::{Task, Todo, Context};


#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);


#[get("/")]
fn get_all(conn: DbConn) -> Json<Context> {
    Json(Task::all(&conn))
}


#[get("/<id>", format = "json")]
fn get_by_id(id: i32, conn: DbConn) -> Json<Task> {
    Json(Task::get_by_id(id, &conn))
}

#[post("/", format = "json", data = "<message>")]
fn create(message: Json<Todo>, conn: DbConn) -> JsonValue {
    Task::insert(message.0, &conn);
    json!({ "status": "ok" })
}

#[put("/<id>", format = "json", data = "<message>")]
fn update(id: i32, message: Json<Todo>, conn: DbConn) -> JsonValue {
    Task::update(id, message.0, &conn);
    json!({ "status": "ok" })
}

#[delete("/<id>", format = "json")]
fn delete(id: i32, conn: DbConn) -> JsonValue {
    Task::delete_with_id(id, &conn);
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
        .mount("/message", routes![get_all, get_by_id, create, update, delete])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
