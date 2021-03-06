// tasksを改変して作成
use diesel::{self, prelude::*};

mod schema {
    table! {
        records {
            id -> Nullable<Integer>,
            wakeupdatetime -> Text,
            condition -> Nullable<Integer>,
            description -> Text,
            isperiod -> Bool,
        }
    }
}

// 自分で↑のスキーマを読んでる？
use self::schema::records;
use self::schema::records::dsl::{records as all_records};

// DBへのCRUD処理を行う構造体の定義
// TODO:Repositoryに移行して、別にEntity構造体を作りたい
#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
pub struct Record {
    pub id: Option<i32>,
    pub wakeupdatetime: String,
    pub condition: Option<i32>,
    pub description: String,
    pub isperiod: bool
}
impl Record {
    // id順にSELECT ALL
    pub fn all(conn: &SqliteConnection) -> Vec<Record> {
        all_records.order(records::id.desc()).load::<Record>(conn).unwrap()
    }
    // idを指定してにSELECT
    pub fn retrieve_by_id(id: i32, conn: &SqliteConnection) -> Option<Vec<Record>> {
        println!("this id is {}", id);
        let record = all_records.find(id).get_result::<Record>(conn);
        if record.is_err() {
            return None;//引っかからなければNone
        }
        Some(vec![record.unwrap()])//recordがあれば返す
    }
    // INSERT処理
    pub fn insert(ffrecord: Record, conn: &SqliteConnection) -> bool {
        let r = Record {
            id: None,
            wakeupdatetime: ffrecord.wakeupdatetime,
            condition: ffrecord.condition,
            description: ffrecord.description,
            isperiod: ffrecord.isperiod
            };
        diesel::insert_into(records::table).values(&r).execute(conn).is_ok()
    }  
    // TODO：日付で検索してIDを返すGET処理
    // pub fn retrieve_id_by_date(){}
    // 
    // UPDATE処理:idを指定して更新を行う処理
    // 実際には指定IDのレコードを削除してから、ID指定で再度レコードを作る処理
    pub fn update_with_id(ffrecord: Record, id: i32, conn: &SqliteConnection) -> bool {
        // 対象id(この時点で所有権はtarget_idに移動？)
        let target_id = id;
        // 指定idのレコードの存在確認
        let _get_res = Record::retrieve_by_id(target_id, conn);
        // --エラーチェック,TODO:ましなやり方を探す
        if _get_res.is_none(){
            return false;
        }
        // レコードの削除
        let _delete_res = Record::delete_with_id(target_id, conn);
        // --エラーチェック,TODO:ましなやり方を探す
        if _delete_res != true {
            return false;
        }
        // 新しいレコードの挿入
        let r = Record {
            id: Some(target_id),
            wakeupdatetime: ffrecord.wakeupdatetime,
            condition: ffrecord.condition,
            description: ffrecord.description,
            isperiod: ffrecord.isperiod
            };
        diesel::insert_into(records::table).values(&r).execute(conn).is_ok()
    }
    // DELETE処理(1):id指定
    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_records.find(id)).execute(conn).is_ok()
    }
    // DELETE処理(2):全指定
    // テストコードでも使ってる？
    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_records).execute(conn).is_ok()
    }
}
