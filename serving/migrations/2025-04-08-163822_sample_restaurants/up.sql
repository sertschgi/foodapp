INSERT INTO users (username, email, password_hash, salt) VALUES 
('sample', 'sample@sample.com', '', '');

INSERT INTO restaurants (name, location) VALUES
('Alpengasthof Lenggrieser Hütte', ST_GeographyFromText('POINT(11.5633 47.7602)')),
('Braustüberl der Tölzer Brauerei', ST_GeographyFromText('POINT(11.5615 47.7608)')),
('Gasthaus Zur Post', ST_GeographyFromText('POINT(11.5608 47.7601)')),
('Restaurant Isarwinkel', ST_GeographyFromText('POINT(11.5589 47.7595)')),
('Café Reith', ST_GeographyFromText('POINT(11.5572 47.7588)')),
('Gut Steinbach', ST_GeographyFromText('POINT(11.5521 47.7523)')),
('Gasthof Jägerwirt', ST_GeographyFromText('POINT(11.5667 47.7634)')),
('Pizzeria Roma', ST_GeographyFromText('POINT(11.5602 47.7605)')),
('China-Restaurant Peking-Ente', ST_GeographyFromText('POINT(11.5597 47.7598)')),
('Eiscafé Venezia', ST_GeographyFromText('POINT(11.5605 47.7603)'));


