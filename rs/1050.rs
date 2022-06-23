use std::collections::HashMap;

fn main() {
	let entrada: i64 = inputline_i64();

	println!("{}", problema(&entrada));
}

fn problema(n: &i64) -> String {
	let mut ddd: HashMap<i64, String> = HashMap::new();
	
	ddd.insert(61, "Brasilia".to_string());
	ddd.insert(71, "Salvador".to_string());
	ddd.insert(11, "Sao Paulo".to_string());
	ddd.insert(21, "Rio de Janeiro".to_string());
	ddd.insert(32, "Juiz de Fora".to_string());
	ddd.insert(19, "Campinas".to_string());
	ddd.insert(27, "Vitoria".to_string());
	ddd.insert(31, "Belo Horizonte".to_string());

	return match ddd.get(n) {
		Some(x) => x.to_owned(),
		None => "DDD nao cadastrado".to_string()
	};
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn string_to_i64(string: String) -> i64 {
	return string
			.trim()
			.parse::<i64>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_i64() -> i64 {
    return string_to_i64(inputline());
            
}