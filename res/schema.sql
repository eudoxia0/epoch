create table day_entries (
    date text primary key,
    content text not null
) strict;

insert into day_entries (
    date,
    content
) values (
    '2026-01-17',
    'Hello, world!'
);
