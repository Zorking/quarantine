Includes CRUD on `/` (GET, POST) and `/<id>` (GET, PUT, DELETE).

POST example: 
`{"description": "example"}`


Deploy: 
1.  Create database `sqlite3 db.sqlite "select sqlite_version();"` 
2.  Run container `sudo docker-compose up`
