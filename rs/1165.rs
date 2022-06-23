fn main() {
    let num_vezes: usize = inputline_usize();
	let mut result: Vec<(usize, bool)> = Vec::new();
	
	for _ in 0..num_vezes {
		let input: usize = inputline_usize();
		let r: bool = problema(input);

		result.push((input, r));
	}

	for i in result {
		if i.1 {
			println!("{} eh primo", i.0);
		}else{
			println!("{} nao eh primo", i.0);
		}
	}
}


fn problema(n: usize) -> bool {
	validar_numero_primo_usize(n)
}

fn validar_numero_primo_usize(n: usize) -> bool {
	if n < 1 {
		return false;
	}

	for i in 2..=(((n as f64).sqrt()) as usize) {
		if n % i == 0 {
			return false;
		}
	}

	return true;
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn string_to_usize(string: String) -> usize {
	return string
			.trim()
			.parse::<usize>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_usize() -> usize {
    return string_to_usize(inputline());
            
}