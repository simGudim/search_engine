-- Your SQL goes here

create table docmaster (
    doc_id serial primary key,
    doc_name varchar not null,
    doc_size int null,
    doc_path varchar not null,
    doc_author varchar null,
    doc_description varchar null,
    doc_association1 varchar null,
    doc_association2 varchar null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);