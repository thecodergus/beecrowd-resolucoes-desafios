type Vetor = Vec<f64>;

fn main() {
    let mut entrada: Vetor = Vec::new();
	let referencia: usize = 100;
	
	for _ in 0..referencia {
		entrada.push(inputline_f64());
	}

	for (index, value) in entrada.iter().enumerate() {
		if *value <= 10.0 {
			println!("A[{}] = {:.1}", index, *value);
		}
	}
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn string_to_f64(string: String) -> f64 {
	return string
			.trim()
			.parse::<f64>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_f64() -> f64 {
    return string_to_f64(inputline());
}