# Timebank

⏰ *Timebank* is a time recording and statistics tool.

🏗️ This project is currently at an early stage.

```text
timebank_core -- Basic structures and functions
timebank_csv  -- Convert csv_data/%Y-%m-%d.csv to timebank.sqlite
timebank_db   -- Database access layer
timebank_http -- HTTP API
```

## About `time_index`

*Timebank* divides 24 hours of the day into 48 time clips, each of which is 30 minutes.

```
time_index = hh * 2 + mm // 30
```

Example:

```
00:00-08:30 => (0+0, 16+1) => (0, 17)
10:30-17:00 => (20+1, 34+0) => (20, 34)
18:00-24:00 => (36+0, 48+0) => (36, 48)
```

## LICENSE

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)
