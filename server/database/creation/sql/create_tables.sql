create table restaurants (
  id uuid default gen_random_uuid() primary key,
  name varchar[40] not null,
  location point not null
);

create table ratings (
  id uuid default gen_random_uuid() primary key,
  restaurant_id uuid references restaurants (id),
  stars int2 check (stars >= 0 AND stars <= 5),
  price int2 check (price >= 0 AND price <= 3)
);
