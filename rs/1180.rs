type Vetor = Vec<i64>;
type Output = (i64, usize);

fn main() {
    let num: usize = inputline_usize();
	let entrada: Vetor = inputline_vector_i64();

	let result: Output = problema(num, entrada);

	println!("Menor valor: {}", result.0);
	println!("Posicao: {}", result.1);
}

fn problema(num: usize, v: Vetor) -> Output {
	let mut menor: (i64, usize) = (v[0], 0);

	for i in 1..num {
		if menor.0 > v[i] {
			menor = (v[i], i);
		} 
	}
	
	return menor;
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

fn inputline_f64() -> f64 {
    return string_to_f64(inputline());
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let p = problema(10, vec![1, 2, 3, 4, -5, 6, 7, 8, 9, 10]);
		let r = (-5, 4);

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let p = problema(10, vec![44, 11,	46,	53,	47,	21,	61,	61,	75,	53]);
		let r = (11, 1);

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let p = problema(5, vec![-1, 0, -1, 5, 9]);
		let r = (-1, 0);

		assert_eq!(p, r);
	}
} 