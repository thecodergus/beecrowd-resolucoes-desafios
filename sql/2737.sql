(select a.name, a.customers_number from lawyers as a order by a.customers_number desc limit 1)
union all
(select a.name, a.customers_number from lawyers as a order by a.customers_number asc limit 1)
union all
(select 'Average' as name, ((select sum(l.customers_number) from lawyers as l) / (select count(*) from lawyers)) as customers_number limit 1);