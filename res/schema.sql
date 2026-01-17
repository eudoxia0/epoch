create table day_entries (
    date text primary key,
    content text not null,
    created_at text not null,
    updated_at text not null
) strict;
