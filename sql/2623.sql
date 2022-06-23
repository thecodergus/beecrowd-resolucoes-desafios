select p.name as name, c.name as name from products as p
inner join categories as c on p.id_categories = c.id
where
p.amount > 100
and
c.id in (1,2,3,6,9)
order by c.id asc;