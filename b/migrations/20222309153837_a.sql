CREATE TYPE SOME_ENUM as ENUM ('A', 'B');

create table other_thing (
    that_enum SOME_ENUM
)
