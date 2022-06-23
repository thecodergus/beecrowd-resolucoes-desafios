type Vetor = Vec<u64>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Exercitos {
	humanos: u64,
	elfos: u64, 
	anoes: u64,
	orcs: u64,
	wargs: u64,
	aguias: u64
}

impl Exercitos{
	fn new(humanos: u64, elfos: u64, anoes: u64, orcs: u64, wargs: u64, aguias: u64) -> Exercitos{
		Exercitos {
			humanos,
			elfos, 
			anoes,
			orcs,
			wargs,
			aguias	
		}
	}

	fn vencedor(&self) -> String {
		if self.humanos + self.elfos + self.anoes + self.aguias >= self.orcs + self.wargs {
			return "Middle-earth is safe.".to_string()
		}
		
		return "Sauron has returned.".to_string()
		
	}
}

fn main() {

	let entrada: Vetor = inputline_vector_u64();

	let batalha = Exercitos::new(entrada[0], entrada[1], entrada[2], entrada[3], entrada[4], entrada[5]);

	println!("{}", batalha.vencedor());
	
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