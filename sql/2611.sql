select m.id as id, m.name as name from movies as m
inner join genres as g on m.id_genres = g.id 
where g.description like '%Action%';