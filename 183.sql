SELECT Customers.name AS Customers
FROM Customers
LEFT JOIN Orders ON Orders.CustomerId = Customers.id
WHERE Orders.id IS NULL