select p.name as name, round(p.salary * 0.1, 2) as tax 
	from people as p
	where p.salary > 3000;