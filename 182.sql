SELECT Person.email
FROM Person
GROUP BY Person.email
HAVING COUNT(Person.email) > 1