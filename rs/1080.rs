type Entrada = Vec<i64>;
type Saida = (i64, i64);


fn main() {
    let mut entrada: Entrada = Vec::new();

	for _ in 0..100 {
		entrada.push(inputline_i64());
	}

	let r: Saida = problema(entrada);

	println!("{}", r.0);
	println!("{}", r.1);
}

fn problema(v: Entrada) -> Saida {
	let maior: i64 = v.iter().max().unwrap().to_owned();
	let mut result = (maior, 0);

	for (index, value) in v.iter().enumerate() {
		if maior == *value {
			result.1 = index as i64 + 1;
			break;
		}
	}

	return result;
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