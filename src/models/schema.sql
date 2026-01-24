create table day_entries (
    date text primary key,
    content text not null,
    word_count integer not null
) strict;

create table achievements (
    achievement_id integer primary key,
    date text not null,
    achievement_type text not null,
    content text not null
);
