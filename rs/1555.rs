#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Valores(i64, i64);
type Aluno = (String, i64);

impl Valores {
	fn quem_ganhou(&self) -> String {
		let rafael = |x: i64, y: i64| (3*x).pow(2) + y.pow(2);
		let beto = |x: i64, y: i64| (2 * x.pow(2)) + (5 * y).pow(2); 
		let carlos = |x: i64, y: i64| (-100*x) + y.pow(3);

		let mut resultado: Vec<Aluno> = Vec::from([
			("Rafael".to_string(), rafael(self.0, self.1)),
			("Beto".to_string(), beto(self.0, self.1)),
			("Carlos".to_string(), carlos(self.0, self.1))
		]);

		resultado.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

		return resultado[2].0.to_owned();
	}
}

fn main() {
    let num: i64 = inputline_i64();
	let mut entrada: Vec<Valores> = Vec::new();
	 
	for _ in 0..num {
		let input: Vec<i64> = inputline_vector_i64();

		entrada.push(Valores(input[0], input[1]));				
	}

	for i in entrada{
		println!("{} ganhou", i.quem_ganhou());
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

fn string_to_i64(string: String) -> i64 {
	return string
			.trim()
			.parse::<i64>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_i64() -> i64 {
    return string_to_i64(inputline());
            
}