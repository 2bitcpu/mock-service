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
