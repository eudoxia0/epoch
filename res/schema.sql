create table day_entries (
    date text primary key,
    content text not null,
    word_count integer not null
) strict;
