#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod record;
#[cfg(test)] mod tests;//testsモジュール

use rocket::Rocket;
use rocket::fairing::AdHoc;//AdHoc？なんだろこれ
use rocket::response::Redirect;
use diesel::SqliteConnection;//まぁわかる
use record::{Record};//importからuse
use rocket_contrib::json::Json;//pythonで言うfrom hoge import hoge as hogeかな

// 記法としては関数型？のマクロ呼び出しで、diesel_migrationsマクロの中にあるらしい
// DRFのmakemigrationみたいなもの？
embed_migrations!();

// DBコネクションの構造体(≒クラス)というのはわかる
// しかし"sqlite_database"はどこから来たんだ
# [database("sqlite_database")]
pub struct DbConn(SqliteConnection);

// 以下、自作のRecordモデルに対する処理
// GET処理：api/v1/records/
# [get("/")]
fn api_records(conn: DbConn) -> Json<Vec<Record>> {
    Json(
        Record::all(&conn)
    )
}
// GET処理：api/v1/records/<id>
# [get("/<id>")]
fn api_records_retrieve_by_id(id: i32, conn: DbConn) -> Option<Json<Vec<Record>>> {
    // match
    let record_retrieve = Record::retrieve_by_id(id, &conn);
    match record_retrieve {
        Some(record_retrieve) => Some(Json(record_retrieve)),
        None => None
    }
}

// CREATE処理：api/v1/records/
# [post("/", format = "json", data = "<json_record>")]
fn api_records_create(json_record: Json<Record>, conn: DbConn) -> Result<Redirect, ()> {
    // Jsonで受け取った値を開く
    let record = json_record.into_inner();
    // Insert処理の実施
    if Record::insert(record, &conn) {
        Ok(Redirect::to("/api/v1/records"))
    } else {
        Err(error!("Failed to create a record."))
    }
}

// UPDATE処理：api/v1/records/<id>
# [put("/<id>", format = "json", data = "<json_record>")]
fn api_records_put_by_id(json_record: Json<Record>, id: i32, conn: DbConn) -> Result<Redirect, ()> {
    // Jsonで受け取った値を開く
    let record = json_record.into_inner();
    if Record::update_with_id(record, id, &conn) {
        Ok(Redirect::to("/api/v1/records"))
    } else {
        Err(error!("Failed to update a record , id: {}", id))
    }
}

// DELETE処理：api/v1/records/<id>
# [delete("/<id>")]
fn api_records_detele_by_id(id: i32, conn: DbConn) -> Result<Redirect, ()> {
    if Record::delete_with_id(id, &conn) {
        Ok(Redirect::to("/api/v1/records"))
    } else {
        Err(error!("Failed to delete a record , id: {}", id))
    }
}

// DBマイグレーション？
fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

// main実行した時に走るやつ？
fn rocket() -> Rocket {
    rocket::ignite()
    .attach(DbConn::fairing())
    .attach(AdHoc::on_attach("DatabaseMigrations", run_db_migrations))
    .mount("/api/v1/records", routes![api_records, api_records_retrieve_by_id, api_records_detele_by_id,api_records_create, api_records_put_by_id])
}

fn main() {
    rocket().launch();
}
