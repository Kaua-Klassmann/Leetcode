SELECT
    Employee.name,
    Bonus.bonus
FROM Employee
LEFT JOIN Bonus ON Bonus.empId = Employee.empId
WHERE Bonus.bonus IS NULL
    OR Bonus.bonus < 1000