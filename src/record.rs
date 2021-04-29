use diesel::{self, prelude::*};

mod schema {
    table! {
        records {
            id -> Nullable<Integer>,
            description -> Text,
            completed -> Bool,
        }
    }
}

use self::schema::records;
use self::schema::records::dsl::{records as all_records, completed as record_completed};

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Record {
    pub id: Option<i32>,
    pub description: String,
    pub completed: bool
}

#[derive(FromForm)]
pub struct Todo {
    pub description: String,
}

impl Record {
    pub fn all(conn: &SqliteConnection) -> Vec<Record> {
        all_records.order(records::id.desc()).load::<Record>(conn).unwrap()
    }

    pub fn insert(todo: Todo, conn: &SqliteConnection) -> bool {
        let t = Record { id: None, description: todo.description, completed: false };
        diesel::insert_into(records::table).values(&t).execute(conn).is_ok()
    }

    pub fn toggle_with_id(id: i32, conn: &SqliteConnection) -> bool {
        let record = all_records.find(id).get_result::<Record>(conn);
        if record.is_err() {
            return false;
        }

        let new_status = !record.unwrap().completed;
        let updated_record = diesel::update(all_records.find(id));
        updated_record.set(record_completed.eq(new_status)).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_records.find(id)).execute(conn).is_ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_records).execute(conn).is_ok()
    }
}
