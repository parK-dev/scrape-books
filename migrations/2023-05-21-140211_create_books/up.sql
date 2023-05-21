create table books (
  id uuid primary key default uuid_generate_v4(),
  title varchar not null unique,
  price float8 not null,
  in_stock bool not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now()
)
