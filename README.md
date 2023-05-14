# Harmoni Server

Back end server for harmoni. 
Provides an http api to access the edgedb database for clients as well as takes care of jwt authentication and logging as well as statistics. 

## Getting Started

You will need edgedb cli installed on your machine.
once you have the database setup you can run the server with the following command:
```bash
JWT_SECRET=<secret> cargo r
```

## JWT authentication 
get the token
```bash
set token (curl -s -X POST -H 'Accept: application/json' -H 'Content-Type: application/json' --data '{"client_id":"cid", "client_secret":"csecret", "username":"bob", "password":"pass"}' http://127.0.0.1:3000/api/authorize | jq -r '.access_token' | sed 's/^/Authorization: Bearer /')
```

use the token
```bash
curl -H $token 127.0.0.1:3000/api/protected
```

have fun...