-- Add some sample ratings for these restaurants
INSERT INTO ratings (restaurant_id, stars, price, rating, author, created_at)
SELECT id, 
       (random() * 2)::int + 3, 
       (random() * 3)::int,
       CASE 
         WHEN random() > 0.7 THEN 'Guter Service!'
         WHEN random() > 0.4 THEN 'Cool'
         WHEN random() > 0.2 THEN 'Voi Leiwand oida!'
         ELSE 'Scheisse'
       END,
      'sample',
       NOW() - (random() * 30 || ' days')::interval
FROM restaurants;

-- Add a few more ratings for some restaurants to have multiple reviews
INSERT INTO ratings (restaurant_id, stars, price, rating, author, created_at)
SELECT id, 
       (random() * 3)::int + 2, 
       (random() * 3)::int,
       CASE 
         WHEN random() > 0.7 THEN 'Konnst fressn'
         WHEN random() > 0.4 THEN 'Alles bis auf das Essen war gut'
         WHEN random() > 0.2 THEN 'Nicht schlecht aber auch nicht gut'
         ELSE 'Nie mehr wieder'
       END,
      'sample',
       NOW() - (random() * 60 || ' days')::interval
FROM restaurants
ORDER BY random() LIMIT 15;
