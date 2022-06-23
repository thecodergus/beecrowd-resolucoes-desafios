select p1.name as name, p2.name as name from products as p1
inner join providers as p2 on p1.id_providers = p2.id
where p2.name like 'Ajax SA';