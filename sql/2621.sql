select p1.name from products as p1
inner join providers as p2 on p1.id_providers = p2.id
where 
p1.amount between 10 and 20
and
p2.name like 'P%';