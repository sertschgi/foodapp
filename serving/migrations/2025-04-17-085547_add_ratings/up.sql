-- Add some sample ratings for these restaurants
INSERT INTO ratings (restaurant_id, stars, price, rating, created_at)
SELECT id, 
       (random() * 2)::int + 3, 
       (random() * 3)::int,
       CASE 
         WHEN random() > 0.7 THEN 'Excellent food and service!'
         WHEN random() > 0.4 THEN 'Good experience overall'
         WHEN random() > 0.2 THEN 'Average, could be better'
         ELSE 'Needs improvement'
       END,
       NOW() - (random() * 30 || ' days')::interval
FROM restaurants;

-- Add a few more ratings for some restaurants to have multiple reviews
INSERT INTO ratings (restaurant_id, stars, price, rating, created_at)
SELECT id, 
       (random() * 3)::int + 2, 
       (random() * 3)::int,
       CASE 
         WHEN random() > 0.7 THEN 'Amazing flavors!'
         WHEN random() > 0.4 THEN 'Decent place for the price'
         WHEN random() > 0.2 THEN 'Not bad but not great'
         ELSE 'Would not return'
       END,
       NOW() - (random() * 60 || ' days')::interval
FROM restaurants
ORDER BY random() LIMIT 15;
