# Rocket Todo Exampleの勉強Repo
* Origin:https://github.com/SergioBenitez/Rocket/tree/master/examples/todo
* exampleを元にFEをVue等に置き換えられるよう、APIサーバ化

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

# DELETE
curl -i -X DELETE  http://localhost:8000/api/v1/records/5


```

# EOF