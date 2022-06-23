type Entrada = Vec<f64>;

fn main() {
    let mut entrada: Entrada = Vec::new();

	loop {
		let input = inputline_f64();

		if input < 0.0 {
			break;
		}

		entrada.push(input);
	}

	println!("{:.2}", problema(entrada.clone()) / entrada.len() as f64);
	
}

fn problema(n: Entrada) -> f64 {
	sum(n)
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

fn sum(mut n: Entrada) -> f64 {
	match n.len() {
		0 => 0.0,
		_ => n.pop().unwrap() + sum(n)
	}
}