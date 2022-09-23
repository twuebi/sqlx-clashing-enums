CREATE TYPE SOME_ENUM as ENUM ('A', 'B');

create table thing (
    some_value SOME_ENUM
)
