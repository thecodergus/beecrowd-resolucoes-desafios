select 
round(
((select sum(p.price) from products as p) 
 / 
 (select count(p.price) from products as p)), 2)
as price;