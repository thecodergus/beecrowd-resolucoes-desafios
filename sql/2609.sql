select c.name as name,
(select sum(p.amount) from products as p where id_categories = c.id) as sum
from categories as c;