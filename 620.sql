SELECT
    Cinema.id,
    Cinema.movie,
    Cinema.description,
    Cinema.rating
FROM Cinema
WHERE Cinema.id & 1 # Checking if column id is odd
    AND Cinema.description != "boring"
ORDER BY Cinema.rating DESC