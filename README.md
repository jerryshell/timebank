# Timebank

⏰ *Timebank* is a time recording and statistics tool.

Web UI: [https://github.com/jerryshell/timebank-web](https://github.com/jerryshell/timebank-web)

```text
timebank_core -- Basic structures and functions
timebank_csv  -- Convert csv_data/%Y-%m-%d.csv to timebank.sqlite
timebank_db   -- Database access layer
timebank_http -- HTTP API
```

## HTTP API

HTTP API usage reference: [timebank-web/src/api](https://github.com/jerryshell/timebank-web/tree/master/src/api)

### `GET /health`

Check server status.

### `GET /record/list`

Get a list of all records.

### `POST /record/search`

Search record.

Request JSON body:

```json
{
    "dateBegin": "yyyy-mm-dd",
    "dateEnd": "yyyy-mm-dd"
}
```

### `POST /record/create`

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

## About `time_index`

*Timebank* divides 24 hours of the day into 48 time clips, each of which is 30 minutes.

```python
# Python code
time_index = hh * 2 + mm // 30
```

Example:

```
00:00-08:30 => (0+0, 16+1) => (0, 17)
10:30-17:00 => (20+1, 34+0) => (20, 34)
18:00-24:00 => (36+0, 48+0) => (36, 48)
```

## How to use `restart.sh`

You first need to put the `timebank_http` binary and `restart.sh` in the same directory.

```bash
./restart.sh timebank_http
```

## LICENSE

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)
