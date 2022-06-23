select
	name 
	 from 
	 ((select concat('Podium: ', l.team) as name, l.position from league as l order by l.position asc limit 3)
		union all
		(select	concat('Demoted: ', l.team) as name, l.position	from league as l order by l.position desc limit 2)) as times
		order by position asc;