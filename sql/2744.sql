select a.id as id, a.password as password, md5(a.password) as MD5
	from account as a;