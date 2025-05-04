INSERT INTO users (username, email, password_hash, salt) VALUES 
('sample', 'sample@sample.com', '', '');

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


