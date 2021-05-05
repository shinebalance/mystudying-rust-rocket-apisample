# Rust学習…Rocket Todo Exampleを元にAPIサーバを作る
* Origin:https://github.com/SergioBenitez/Rocket/tree/master/examples/todo

## モチベーション
* Rustlangの学習。Python以外の下地を作るための日曜大工。
* Webフレームワークは他言語(Python)である程度仕組みを知っており、題材に適切と判断。

## やっていること、やろうとしていること
* Rocket公式のtodo exampleを元に、CRUD機能のあるAPIサーバを作る。
  * FEをJSの別ライブラリで作ることを想定している
  * 元々存在するtodoとは全く別にテーブル(`records`)を作ってそこに対するリクエストのルーティングとCRUD処理を実装する
    * 起床時間のレコーディングアプリのようなものを想定している
* なるべくTDDで作る。
  * 手動でやっているような操作確認はなるべくテストコード化する。


## Commands
```
# ビルド、実行系
cargo build
cargo run
# テストコード実行
cargo test
```

### curl CRUD tests
```
# POST
curl -i -H "Content-Type: application/json" -X POST -d '{"wakeupdatetime":"2021-05-05 07:00:00","condition":5,"description":"POSTdemo","isperiod":false}' http://localhost:8000/api/v1/records

# GET
curl http://localhost:8000/api/v1/records

# UPDATE
curl -i -H "Content-Type: application/json" -X PUT -d '{"wakeupdatetime":"2021-05-05 17:00:00","condition":5,"description":"UPDATED demo","isperiod":false}' http://localhost:8000/api/v1/records/4

# DELETE
curl -i -X DELETE  http://localhost:8000/api/v1/records/5

```

## TODO
 - [ ]:実行失敗時のHTTPステータスのコントロール
 - [ ]:エラーハンドリングの実装
 - [ ]:CRUDのテストコード実装


# EOF