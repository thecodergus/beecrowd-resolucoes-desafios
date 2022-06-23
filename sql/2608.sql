select 
	(select max(p.price) as price from products as p limit 1) as price,
	(select min(p.price) as price from products as p limit 1) as price;