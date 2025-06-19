SELECT Employee.name AS Employee
FROM Employee
JOIN Employee Manager ON Manager.id = Employee.managerId
WHERE Employee.salary > Manager.salary