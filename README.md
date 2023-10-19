# http-client

## introduce
This is a trust learning project similar to curl。 currently only supporting HTTP (GET, POST, PUT, DELETE)。

OPTIONS has the same effect as curl:
- -0
- -X
- -H
- -d
- -b

## for example
```shell
http-client.exe -X GET http://192.168.11.31:8888
http-client.exe -X POST http://127.0.0.1:8888/json -d "{\"a\": 1}"
```