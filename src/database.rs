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

#[derive(FromForm)]
pub struct Todo {
    pub description: String,
}

impl Task {
    pub fn all(conn: &SqliteConnection) -> Vec<Task> {
        all_tasks.order(tasks::id.desc()).load::<Task>(conn).unwrap()
    }

    pub fn last(conn: &SqliteConnection) -> Vec<Task> {
        all_tasks.order(tasks::id.desc()).load::<Task>(conn).unwrap()
    }

    pub fn insert(todo: Todo, conn: &SqliteConnection) -> bool {
        let t = Task { id: None, description: todo.description};
        diesel::insert_into(tasks::table).values(&t).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_tasks.find(id)).execute(conn).is_ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_tasks).execute(conn).is_ok()
    }
}
