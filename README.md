保存先やホストを変更する場合は環境変数を設定

MOCK_HOST  
規定値: "0.0.0.0:3000"


MOCK_DATA_DIR  
規定値: "./service"  
相対パスの場合、実行ファイルがある場所ではなく実行した場所からの相対パスなので注意

環境変数を設定して起動する例
```
export MOCK_HOST="0.0.0.0:3033" && export MOCK_DATA_DIR="./data" && ./target/release/mock-service
```

```
curl -i -X POST -H "Content-Type: application/json" \
-d '{"endPoint":"text/test","statusCode":200,"contentType":"text/plain","content":"Hello,World!"}' \
http://localhost:3000/manage-mock

curl -i http://localhost:3000/service/text/test


curl -i -X POST -H "Content-Type: application/json" \
-d '{"endPoint":"json/test","statusCode":200,"contentType":"application/json","content":"{\"message\":\"Hello,World!\"}"}' \
http://localhost:3000/manage-mock

curl -i http://localhost:3000/service/json/test

curl -i -X POST -H "Content-Type: application/json" \
-d '{"endPoint":"error/test","statusCode":401,"contentType":"application/json","content":"{}"}' \
http://localhost:3000/manage-mock

curl -i http://localhost:3000/service/error/test
```
