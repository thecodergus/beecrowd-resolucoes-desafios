type Vetor = Vec<i64>;

fn main() {
    let mut entrada: Vetor = Vec::new();
	let referencia: usize = 20;

	
	for _ in 0..referencia {
		entrada.push(inputline_i64());
	}

	for i in (0..referencia).into_iter().rev() {
		println!("N[{}] = {}", referencia - 1 - i, entrada[i]);
	}
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