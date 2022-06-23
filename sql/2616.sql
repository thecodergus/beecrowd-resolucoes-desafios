select c.id, c.name from customers as c 
where 
c.id not in 
	(select c1.id from customers as c1
	 	inner join locations as l1 on c1.id = l1.id_customers);