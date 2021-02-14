-- Your SQL goes here
create extension if not exists "uuid-ossp";

create table users (
    id SERIAL PRIMARY KEY, 
    user_id uuid default uuid_generate_v4(),
    username varchar not null unique,
    email varchar not null unique,
    password_hash varchar not null,
    full_name varchar null,
    bio varchar null,
    image varchar null,
    -- email_verified,
    -- active 
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);