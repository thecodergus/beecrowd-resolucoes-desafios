type Vetor = Vec<i64>;

fn main() {
	let mut entrada: Vec<String> = Vec::new();
	
    loop {
		let input: Vetor = inputline_vector_i64();

		if input[0] == input[1] {
			break;
		}else if input[0] < input[1]{
			entrada.push("Crescente".to_string());
		}else{
			entrada.push("Decrescente".to_string());
		}
	}

	for i in entrada {
		println!("{}", i);
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