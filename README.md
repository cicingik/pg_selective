<h1 align="center">pg_selective</h1>
<p align="center">
  <em> Just `pg_dump` for table with condition. </em>
</p>


### Target Commands

```shell
Usage: pg_selective [OPTIONS] --host <HOST> --port <PORT> \
        --username <USERNAME> --password <PASSWORD> \
        --dbname <DBNAME> --schema <SCHEMA> --table-name <TABLE_NAME>
```

Available Options:
```shell
Connection options:
  -H, --host <HOST>              Database server host or socket dirctory
  -P, --port <PORT>              Database server port number
  -U, --username <USERNAME>      Connect as specified database user
  -W, --password <PASSWORD>      Force password prompt (should happen automaticaly)
  -d, --dbname <DBNAME>          Database to dump
  -s, --schema <SCHEMA>          Database schema
  -t, --table-name <TABLE_NAME>  table name

General options:
  -f, --file <FILE>              Output file nam
  -v, --verbose                  Verbose mode
  -V, --version                  Print version
  -h, --help                     Print help


Options controlling the output content:
  -c, --condition <CONDITION>    Condition query
  -E, --encoding <ENCODING>      Dump the data in encoding ENCODING
  -O, --no-owner                 Skip restoration of object ownership in plain-text format
  -a, --data-only                Dump only the data, not the schema
  --column-inserts               Dump data as INSERT commands with column names
  --if-exists                    Use IF EXISTS when dropping objects
  --inserts                      Dump data as INSERT commands, rather than COPY
```
