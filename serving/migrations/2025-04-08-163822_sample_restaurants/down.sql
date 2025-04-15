--- Begin transaction for safety
BEGIN;

-- Delete the sample restaurants
DELETE FROM restaurants 
WHERE name IN (
  'The Gourmet Kitchen',
  'Seaside Bistro',
  'Urban Grill',
  'Mountain View Eats',
  'Riverside Cafe',
  'Downtown Delights',
  'Harbor House',
  'The Cozy Corner',
  'Pasta Paradise',
  'Sunset Grill'
);

-- Verify deletions (optional - can be removed)
SELECT 'Remaining restaurants:' AS message, COUNT(*) FROM restaurants;
SELECT 'Remaining ratings:' AS message, COUNT(*) FROM ratings;

-- Commit the transaction
COMMIT;- This file should undo anything in `up.sql`
