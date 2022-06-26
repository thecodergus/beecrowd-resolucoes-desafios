type Notas = Vec<u64>;

fn main() {
    let num_casos: u64 = inputline_u64();
	let mut result: Vec<f64> = Vec::new();

	for _ in 0..num_casos {
		let mut entrada: Vec<u64> = inputline_vector_u64();
		let num: u64 = *entrada.first().unwrap();
		// entrada.remove(0);
		entrada[0] = 0;
	
		result.push(problema(num, entrada));
	}

	for i in result{
		println!("{:.3}%", i);
	}
	
}

fn problema(num_alunos: u64, notas: Notas) -> f64 {
	let media: f64 = notas.iter().sum::<u64>() as f64 / num_alunos as f64;
	let mut cont = 0;

	for i in notas{
		if i as f64 > media {
			cont += 1;
		}
	}

	return (cont as f64 * 100.0) / num_alunos as f64;	
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn string_to_u64(string: String) -> u64 {
	return string
			.trim()
			.parse::<u64>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_u64() -> u64 {
    return string_to_u64(inputline());
            
}

fn splitline(string: String) -> Vec<String> {
	let itens: Vec<&str> = string.split_whitespace().collect();

	let mut result: Vec<String> = Vec::new();

	for i in itens {
		result.push(i.to_string());
	}
	
	return result;
}

fn convert_vector_string_to_u64(vetor: Vec<String>) -> Vec<u64> {
	let mut result: Vec<u64> = Vec::new();

	for i in vetor {
		result.push(i.to_string().parse::<u64>().unwrap());
	}

	return result;
}

fn inputline_vector_u64() -> Vec<u64> {
	return convert_vector_string_to_u64(splitline(inputline()));
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let n: Notas = vec![50, 50, 70, 80, 100];
		
		let p: f64 = problema(5, n);
		let r: f64 = 40.000;

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let n: Notas = vec![100, 95, 90, 80, 70, 60, 50];
		
		let p: f64 = problema(7, n);
		let r: f64 = 57.143;

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let n: Notas = vec![70, 90, 80];
		
		let p: f64 = problema(3, n);
		let r: f64 = 33.333;

		assert_eq!(p, r);
	}

	#[test]
	fn test_4() {
		let n: Notas = vec![70, 90, 81];
		
		let p: f64 = problema(3, n);
		let r: f64 = 66.667;

		assert_eq!(p, r);
	}

	#[test]
	fn test_5() {
		let n: Notas = vec![100, 99, 98, 97, 96, 95, 94, 93, 91];
		
		let p: f64 = problema(9, n);
		let r: f64 = 55.556;

		assert_eq!(p, r);
	}
} 