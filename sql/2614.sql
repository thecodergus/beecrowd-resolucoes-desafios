select c.name, r.rentals_date from customers as c
inner join rentals as r on c.id = r.id_customers
where 
extract(month from r.rentals_date) = '09' and
extract(year from r.rentals_date) = '2016';