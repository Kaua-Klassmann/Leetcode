SELECT
    Products.product_name,
    SUM(Orders.unit) AS unit
FROM Products
JOIN Orders ON Orders.product_id = Products.product_id
WHERE Orders.order_date BETWEEN "2020-02-01" AND "2020-02-29"
GROUP BY Orders.product_id
HAVING unit >= 100