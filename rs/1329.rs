type Vetor = Vec<bool>;
type Resultado = (usize, usize);

fn main() {
    let mut entrada: Vetor = Vec::new();
	let mut resultado: Vec<Resultado> = Vec::new();

	loop {
		let input: usize = inputline_usize();
		let mut r: Resultado = (0, 0);

		if input == 0 {
			break;
		}

		let jogadas: Vetor = inputline().split_whitespace()
											.map(|x| *x.to_string() == '1'.to_string())
											.collect();

		for i in 0..input {
			if !jogadas[i]{
				r.0 += 1;
			}else{
				r.1 += 1;
			}
		}

		resultado.push(r);
	}

	for i in resultado {
		println!("Mary won {} times and John won {} times", i.0, i.1);
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