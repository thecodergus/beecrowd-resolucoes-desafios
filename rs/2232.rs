fn main() {
    let num_casos: usize = inputline_usize() as usize;
	let mut resultado: Vec<usize> = Vec::new();
	
	for _ in 0..num_casos {
		let input: usize = inputline_usize() as usize;

		resultado.push(problema(input - 1));
	}

	for i in resultado {
		println!("{}", i);
	}
}

fn problema(n: usize) -> usize {
	match n {
		0 => 1,
		_ => (2 as usize).pow(n as u32) + problema(n - 1)
	}
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