type Notas = Vec<f64>;

fn main() {
    let casos: i64 =  inputline_i64();
	let mut entrada: Notas = Vec::new();
	
	for _ in 0..casos {
		entrada.push(problema(inputline_vector_f64()));	
	}

	for i in entrada {
		println!("{:.1}", i);
	}
}

fn problema(notas: Notas) -> f64 {
	let mult = |x, y, z| x*2.0 + y*3.0 + z*5.0;
	
	return mult(notas[0], notas[1], notas[2]) / 10 as f64;
}

#[test]
fn caso_1() {
	assert_eq!(problema(vec![6.5, 4.3, 6.2]), 5.7);
}

#[test]
fn caso_2() {
	assert_eq!(problema(vec![5.1, 4.2, 8.1]), 6.3);
}

#[test]
fn caso_3() {
	assert_eq!(problema(vec![8.0, 9.0, 10.0]), 9.3);
}

#[test]
fn caso_4() {
	assert_eq!(problema(vec![1.0, 1.3, 1.0]), 1.1);
}

#[test]
fn caso_5() {
	assert_eq!(problema(vec![0.1, 0.0, 0.0]), 0.0);
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

fn convert_vector_string_to_f64(vetor: Vec<String>) -> Vec<f64> {
	let mut result: Vec<f64> = Vec::new();

	for i in vetor {
		result.push(i.to_string().parse::<f64>().unwrap());
	}

	return result;
}

fn inputline_vector_f64() -> Vec<f64> {
	return convert_vector_string_to_f64(splitline(inputline()));
}

fn string_to_i64(string: String) -> i64 {
	return string
			.trim()
			.parse::<i64>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_i64() -> i64 {
    return string_to_i64(inputline());
            
}