select lr.name as name, round(lr.omega * 1.618, 3) as "Fator N"
	from life_registry as lr 
	where lr.name like 'Richard%' and dimensions_id in (select d.id from dimensions as d where name in ('C875', 'C774'))
	order by "Fator N" asc