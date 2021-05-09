extern crate parking_lot;
extern crate rand;

use super::record::Record;//追加
use super::task::Task;
use self::parking_lot::Mutex;
// use self::rand::{Rng, thread_rng, distributions::Alphanumeric};

use rocket::local::Client;
use rocket::http::ContentType;
// use rocket::http::{Status, ContentType};

// We use a lock to synchronize between tests so DB operations don't collide.
// For now. In the future, we'll have a nice way to run each test in a DB
// transaction so we can regain concurrency.
static DB_LOCK: Mutex<()> = Mutex::new(());

macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => ({
        let _lock = DB_LOCK.lock();
        let rocket = super::rocket();
        let db = super::DbConn::get_one(&rocket);
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.expect("failed to get database connection for testing");
        assert!(Task::delete_all(&$conn), "failed to delete all tasks for testing");

        $block
    })
}

// #[test]
// fn test_insertion_deletion() {
//     run_test!(|client, conn| {
//         // Get the tasks before making changes.
//         let init_tasks = Task::all(&conn);

//         // Issue a request to insert a new task.
//         client.post("/todo")
//             .header(ContentType::Form)
//             .body("description=My+first+task")
//             .dispatch();

//         // Ensure we have one more task in the database.
//         let new_tasks = Task::all(&conn);
//         assert_eq!(new_tasks.len(), init_tasks.len() + 1);

//         // Ensure the task is what we expect.
//         assert_eq!(new_tasks[0].description, "My first task");
//         assert_eq!(new_tasks[0].completed, false);

//         // Issue a request to delete the task.
//         let id = new_tasks[0].id.unwrap();
//         client.delete(format!("/todo/{}", id)).dispatch();

//         // Ensure it's gone.
//         let final_tasks = Task::all(&conn);
//         assert_eq!(final_tasks.len(), init_tasks.len());
//         if final_tasks.len() > 0 {
//             assert_ne!(final_tasks[0].description, "My first task");
//         }
//     })
// }

// #[test]
// fn test_toggle() {
//     run_test!(|client, conn| {
//         // Issue a request to insert a new task; ensure it's not yet completed.
//         client.post("/todo")
//             .header(ContentType::Form)
//             .body("description=test_for_completion")
//             .dispatch();

//         let task = Task::all(&conn)[0].clone();
//         assert_eq!(task.completed, false);

//         // Issue a request to toggle the task; ensure it is completed.
//         client.put(format!("/todo/{}", task.id.unwrap())).dispatch();
//         assert_eq!(Task::all(&conn)[0].completed, true);

//         // Issue a request to toggle the task; ensure it's not completed again.
//         client.put(format!("/todo/{}", task.id.unwrap())).dispatch();
//         assert_eq!(Task::all(&conn)[0].completed, false);
//     })
// }

// #[test]
// fn test_many_insertions() {
//     const ITER: usize = 100;

//     let mut rng = thread_rng();
//     run_test!(|client, conn| {
//         // Get the number of tasks initially.
//         let init_num = Task::all(&conn).len();
//         let mut descs = Vec::new();

//         for i in 0..ITER {
//             // Issue a request to insert a new task with a random description.
//             let desc: String = rng.sample_iter(&Alphanumeric).take(12).collect();
//             client.post("/todo")
//                 .header(ContentType::Form)
//                 .body(format!("description={}", desc))
//                 .dispatch();

//             // Record the description we choose for this iteration.
//             descs.insert(0, desc);

//             // Ensure the task was inserted properly and all other tasks remain.
//             let tasks = Task::all(&conn);
//             assert_eq!(tasks.len(), init_num + i + 1);

//             for j in 0..i {
//                 assert_eq!(descs[j], tasks[j].description);
//             }
//         }
//     })
// }

// #[test]
// fn test_bad_form_submissions() {
//     run_test!(|client, _conn| {
//         // Submit an empty form. We should get a 422 but no flash error.
//         let res = client.post("/todo")
//             .header(ContentType::Form)
//             .dispatch();

//         let mut cookies = res.headers().get("Set-Cookie");
//         assert_eq!(res.status(), Status::UnprocessableEntity);
//         assert!(!cookies.any(|value| value.contains("error")));

//         // Submit a form with an empty description. We look for 'error' in the
//         // cookies which corresponds to flash message being set as an error.
//         let res = client.post("/todo")
//             .header(ContentType::Form)
//             .body("description=")
//             .dispatch();

//         let mut cookies = res.headers().get("Set-Cookie");
//         assert!(cookies.any(|value| value.contains("error")));

//         // Submit a form without a description. Expect a 422 but no flash error.
//         let res = client.post("/todo")
//             .header(ContentType::Form)
//             .body("evil=smile")
//             .dispatch();

//         let mut cookies = res.headers().get("Set-Cookie");
//         assert_eq!(res.status(), Status::UnprocessableEntity);
//         assert!(!cookies.any(|value| value.contains("error")));
//     })
// }

// 以降、Record処理用に自分で追加
#[test]
fn test_get_record() {
    run_test!(|client, conn| {
        // レコード数のチェック
        let init_records = Record::all(&conn);
        // Getできることの確認
        let _req = client.get("/api/v1/records");
        // id:1を使って単体レコードの取得テスト
        // TODO:動的にidを取る方法があれば別途
        let check_id = 1;
        let retrieved_records = Record::retrieve_by_id(check_id, &conn).unwrap();
        if retrieved_records.len() != 0 {
            // get処理
            let url = format!("/api/v1/records/{}", check_id);
            client.get(url);
            assert_eq!(retrieved_records.len(), 1);
        }
        // レコード数の存在確認(今の時点であまり意味はない)
        let new_records = Record::all(&conn);
        assert_eq!(new_records.len(), init_records.len());

    })
}

#[test]
fn test_create_record() {
    run_test!(|client, conn| {
        // レコード数のチェック
        let init_records = Record::all(&conn);
        // パラメータ
        let param_1 = "2021-05-05 07:00:00";
        let param_2 = "3";
        let param_3 = "created by test code";
        let param_4 = false;
        // let mut params_map = HashMap::new();
        // params_map.insert("wakeupdatetime", param_1);
        // params_map.insert("condition", param_2);
        // params_map.insert("description", param_3);
        // params_map.insert("isperiod", param_4);
        let json_params = format!(
            r#"{{
                "wakeupdatetime": {},
                "condition": {}
                "description": {}
                "isperiod": {}
            }}"#, param_1, param_2, param_3, param_4
        );
        // let json_params = json!(
        //     {
        //         "wakeupdatetime": "2021-05-05 07:00:00",
        //         "condition": "3",
        //         "description": "created by test code",
        //         "isperiod": false
        //     });
        // let body_params = serde_json::to_value(&json_params);
        // レコード作成
        let _req = client.post("/api/v1/records")
        .header(ContentType::JSON)
        .body(json_params);
        // .body(json_params);
        // レコード数でチェック
        let now_records = Record::all(&conn);
        assert_eq!(now_records.len(), init_records.len());

    })
}