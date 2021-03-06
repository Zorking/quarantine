use diesel::{self, prelude::*};

mod schema {
    table! {
        tasks {
            id -> Nullable<Integer>,
            description -> Text,
        }
    }
}

use self::schema::tasks;
use self::schema::tasks::dsl::{tasks as all_tasks};

#[table_name="tasks"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Task {
    pub id: Option<i32>,
    pub description: String,
}

#[table_name="tasks"]
#[derive(Serialize, Deserialize, AsChangeset)]
pub struct Todo {
    pub description: String,
}

#[derive(Serialize)]
pub struct Context { tasks: Vec<Task> }

impl Task {
    pub fn all(conn: &SqliteConnection) -> Context {
        Context { tasks: all_tasks.order(tasks::id.desc()).load::<Task>(conn).unwrap()}
    }

    pub fn get_by_id(id: i32, conn: &SqliteConnection) -> Task {
        all_tasks.find(id).get_result::<Task>(conn).unwrap()
    }

    pub fn insert(todo: Todo, conn: &SqliteConnection) -> bool {
        let t = Task { id: None, description: todo.description};
        diesel::insert_into(tasks::table).values(&t).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_tasks.find(id)).execute(conn).is_ok()
    }

    pub fn update(id: i32, todo: Todo, conn: &SqliteConnection) -> bool {
        diesel::update(all_tasks.find(id))
            .set(&todo)
            .execute(conn)
            .is_ok()
    }
}
