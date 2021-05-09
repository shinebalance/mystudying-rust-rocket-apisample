extern crate parking_lot;
extern crate rand;

use super::record::Record;
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
        // assert!(Task::delete_all(&$conn), "failed to delete all tasks for testing");

        $block
    })
}


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