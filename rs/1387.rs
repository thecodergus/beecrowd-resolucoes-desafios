type Matriz = Vec<Vec<i64>>;

fn main() {
	let mut entrada: Matriz = Vec::new();
	
    loop {
		let input: Vec<i64> = inputline_vector_i64();
		
		if input[0] == 0 && input[1] == 0 {
			break;
		}

		entrada.push(input);
	}

	for i in entrada {
		println!("{}", problema(i[0], i[1]));
	}
}

fn problema(a: i64, b: i64) -> i64 {
	a + b
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(problema(2, 5), 7);
		assert_eq!(problema(3, 3), 6);
		assert_eq!(problema(2, -5), -3);
	}
} 