# Clashing Enums

Run

```
$ docker run -d -e POSTGRES_USER="postgres"  -e POSTGRES_PASSWORD="password"     -p 5432:5432       postgres:13.5
$ sleep 0.5
$ cargo run -p migrate
$ cargo run -p a
$ cargo run -p b
```

or `repro.sh` to reproduce the issue, you should see something along the lines of:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Database(PgDatabaseError { severity: Error, code: "42804", message: "column \"that_enum\" is of type some_enum but expression is of type a.some_enum", detail: None, hint: Some("You will need to rewrite or cast the expression."), position: Some(Original(44)), where: None, schema: None, table: None, column: None, data_type: None, constraint: None, file: Some("parse_target.c"), line: Some(581), routine: Some("transformAssignedExpr") })', b/src/main.rs:32:6
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

if `cargo run -p a` or `cargo run -p b` depends on the ordering of the rows returned by this [query](https://github.com/launchbadge/sqlx/blob/061fdcabd72896d9bc3abb4ea4af6712a04bc0a8/sqlx-core/src/postgres/connection/describe.rs#L349) which fetches the OID of a type based on its name from the global pg_enum table.
