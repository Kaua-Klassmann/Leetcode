SELECT Courses.class
FROM Courses
GROUP BY Courses.class
HAVING COUNT(Courses.class) >= 5