#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod task;
#[cfg(test)] mod tests;//testsモジュール？


use rocket::Rocket;
use rocket::fairing::AdHoc;//AdHoc？なんだろこれ
use rocket::request::{Form, FlashMessage};//これは何となく分かりそう
use rocket::response::{Flash, Redirect};//これも
use rocket_contrib::{templates::Template, serve::StaticFiles};//静的ファイルの配置機能？
use diesel::SqliteConnection;//まぁわかる

use task::{Task, Todo};//同じフォルダから

// 何か元コードには色々書いてるんだよな
// 記法としては関数型？のマクロ呼び出しで、diesel_migrationsマクロの中にあるらしい
// DRFのmakemigrationみたいなもの？
embed_migrations!();

// DBコネクションの構造体(≒クラス)というのはわかる
// しかし"sqlite_database"はどこから来たんだ
# [database("sqlite_database")]
pub struct DbConn(SqliteConnection);

// 構造体(Class)Contextの宣言
// どうもシリアライザ通す構造体っぽいけど
# [derive(Debug, Serialize)]
struct Context<'a, 'b>{ 
    msg: Option<(&'a str, &'b str)>, tasks: Vec<Task>
}

// implってなんだ？→メソッドの定義やね
impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &DbConn, msg: &'a str) -> Context<'static, 'a> {
        Context{
            msg: Some(("error", msg)),
            tasks: Task::all(conn)}
    }

    pub fn raw(conn: &DbConn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{msg: msg, tasks: Task::all(conn)}
    }
}

// POST
# [post("/", data = "<todo_form>")]
fn new(todo_form: Form<Todo>, conn: DbConn) -> Flash<Redirect> {
    let todo = todo_form.into_inner();//受け取ったtodo_formの中身を渡していると思う
    // 入力が空の場合
    if todo.description.is_empty() {
        Flash::error(Redirect::to("/"), "Description kara ni sinai de ne")
    // Taskマクロ？からエラーが帰った場合
    } else if Task::insert(todo, &conn) {
        Flash::success(Redirect::to("/"), "Successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Could not be inserted due an internal error.")
    }
}

// UPDATE
# [put("/<id>")]
fn toggle(id: i32, conn: DbConn) -> Result<Redirect, Template> {
    if Task::toggle_with_id(id, &conn) {
        Ok(Redirect::to("/"))
    } else {
        Err(Template::render("index", &Context::err(&conn, "Couldn't toggle task.")))
    }
}

// DELETE
# [delete("/<id>")]
fn delete(id: i32, conn: DbConn) -> Result<Flash<Redirect>, Template> {
    if Task::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/"), "Todo was deleted."))
    } else {
        Err(Template::render("index", &Context::err(&conn, "Couldn't delete task.")))
    }
}

// GET
# [get("/")]
fn index(msg: Option<FlashMessage>, conn: DbConn) -> Template {
    Template::render("index", &match msg {
        Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
        None => Context::raw(&conn, None),
    })
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
    .mount("/", StaticFiles::from("static/"))
    .mount("/", routes![index])
    .mount("/todo", routes![new, toggle, delete]) //マウント…？
    .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
