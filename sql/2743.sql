select p.name as name, char_length(p.name) as "lenght" 
	from people as p
	order by "lenght" desc;