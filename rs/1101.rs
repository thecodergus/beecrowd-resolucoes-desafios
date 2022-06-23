type Vetor = Vec<i64>;
type Output = (Vetor, i64);

fn main() {
    let mut result: Vec<Output> = Vec::new();

	loop {
		let mut entrada: Vec<i64> = inputline_vector_i64();

		if entrada[0] <= 0 || entrada[1] <= 0 {
			break;
		}

		entrada.sort();

		let aux: Vec<i64> = (entrada[0]..=entrada[1]).collect();
		
		let soma: i64 = aux.iter().sum();

		result.push((aux, soma));
	}

	for i in result {
		for j in i.0 {
			print!("{} ", j);
		}
		println!("Sum={}", i.1);
	}
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn splitline(string: String) -> Vec<String> {
	let itens: Vec<&str> = string.split_whitespace().collect();

	let mut result: Vec<String> = Vec::new();

	for i in itens {
		result.push(i.to_string());
	}
	
	return result;
}

fn convert_vector_string_to_i64(vetor: Vec<String>) -> Vec<i64> {
	let mut result: Vec<i64> = Vec::new();

	for i in vetor {
		result.push(i.to_string().parse::<i64>().unwrap());
	}

	return result;
}

fn inputline_vector_i64() -> Vec<i64> {
	return convert_vector_string_to_i64(splitline(inputline()));
}