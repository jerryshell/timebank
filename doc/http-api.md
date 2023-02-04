# HTTP API

HTTP API usage reference: [timebank-web/src/api](https://github.com/jerryshell/timebank-web/tree/master/src/api)

## GET `/health`

Check server status.

## GET `/record/list`

Get a list of all records.

## POST `/record/search`

Search record.

Request JSON body:

```json
{
    "dateBegin": "yyyy-mm-dd",
    "dateEnd": "yyyy-mm-dd"
}
```

## POST `/record/create`

Create record.

You need to add `admin_token` to http header.

`admin_token` can be customized in environment variables(ADMIN_TOKEN), example: [restart.sh](restart.sh)

Request JSON body:

```json
{
    "date": "yyyy-mm-dd",
    "timeIndexBegin": 1,
    "timeIndexEnd": 12,
    "type": "string",
    "remark": "string"
}
```
