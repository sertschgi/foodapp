-- Insert 10 sample restaurants with geographic locations in major cities
INSERT INTO restaurants (name, location) VALUES
('The Gourmet Kitchen', ST_GeographyFromText('POINT(-73.9857 40.7484)')), -- New York
('Seaside Bistro', ST_GeographyFromText('POINT(-118.4085 33.9416)')), -- Los Angeles
('Urban Grill', ST_GeographyFromText('POINT(-87.6298 41.8781)')), -- Chicago
('Mountain View Eats', ST_GeographyFromText('POINT(-122.4194 37.7749)')), -- San Francisco
('Riverside Cafe', ST_GeographyFromText('POINT(-77.0369 38.9072)')), -- Washington DC
('Downtown Delights', ST_GeographyFromText('POINT(-74.0060 40.7128)')), -- New York
('Harbor House', ST_GeographyFromText('POINT(-118.2673 34.0537)')), -- Los Angeles
('The Cozy Corner', ST_GeographyFromText('POINT(-95.3698 29.7604)')), -- Houston
('Pasta Paradise', ST_GeographyFromText('POINT(-71.0589 42.3601)')), -- Boston
('Sunset Grill', ST_GeographyFromText('POINT(-112.0740 33.4484)')); -- Phoenix

-- -- Add some sample ratings for these restaurants
-- INSERT INTO ratings (restaurant_id, stars, price, rating, created_at)
-- SELECT id, 
--        (random() * 5)::int + 1, 
--        (random() * 3)::int + 1,
--        CASE 
--          WHEN random() > 0.7 THEN 'Excellent food and service!'
--          WHEN random() > 0.4 THEN 'Good experience overall'
--          WHEN random() > 0.2 THEN 'Average, could be better'
--          ELSE 'Needs improvement'
--        END,
--        NOW() - (random() * 30 || ' days')::interval
-- FROM restaurants;
--
-- -- Add a few more ratings for some restaurants to have multiple reviews
-- INSERT INTO ratings (restaurant_id, stars, price, rating, created_at)
-- SELECT id, 
--        (random() * 5)::int + 1, 
--        (random() * 3)::int + 1,
--        CASE 
--          WHEN random() > 0.7 THEN 'Amazing flavors!'
--          WHEN random() > 0.4 THEN 'Decent place for the price'
--          WHEN random() > 0.2 THEN 'Not bad but not great'
--          ELSE 'Would not return'
--        END,
--        NOW() - (random() * 60 || ' days')::interval
-- FROM restaurants
-- ORDER BY random() LIMIT 15;
