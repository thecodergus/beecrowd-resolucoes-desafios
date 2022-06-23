trait Experiencias {
	fn new() -> Cobaias;
	fn add(&mut self, linha: String);
	fn get_total_cobaias(&self) -> u64;
	fn get_porcentagens(&self) -> (f64, f64, f64);
} 

struct Cobaias {
	sapo: u64,
	rato: u64,
	coelho: u64	
}

type Percentuais = (f64, f64, f64);

impl Experiencias for Cobaias {
	fn new() -> Cobaias {
		return Cobaias {
			sapo: 0,
			rato: 0,
			coelho: 0
		}
	}

	fn add(&mut self, linha: String) {
		let itens: Vec<&str> = linha.split_whitespace().collect();
		
		let (quantidade, animal): (u64, &str) = (
			itens[0].trim().parse().unwrap(),
			itens[1]
		);


		match animal {
			"C" => self.coelho += quantidade,
			"R" => self.rato += quantidade,
			"S" => self.sapo += quantidade,
			_ => ()
		};
	}

	fn get_total_cobaias(&self) -> u64 {
		return self.sapo + self.rato + self.coelho;
	}

	fn get_porcentagens(&self) -> Percentuais {
		let total_animais: f64 = self.get_total_cobaias() as f64;
		let porcentagem = |x: f64| (x*100.0) / total_animais;

		return (
			porcentagem(self.coelho as f64),
			porcentagem(self.rato as f64),
			porcentagem(self.sapo as f64)
		)
	}
}


fn main() {
    let num_vezes: u64 = inputline_u64();
	let mut laboratorio: Cobaias = Cobaias::new();

	for _ in 0..num_vezes {
		laboratorio.add(inputline());
	}

	let p: Percentuais = laboratorio.get_porcentagens();

	println!("Total: {} cobaias", laboratorio.get_total_cobaias());
	println!("Total de coelhos: {}", laboratorio.coelho);
	println!("Total de ratos: {}", laboratorio.rato);
	println!("Total de sapos: {}", laboratorio.sapo);
	println!("Percentual de coelhos: {:.2} %", p.0);
	println!("Percentual de ratos: {:.2} %", p.1);
	println!("Percentual de sapos: {:.2} %", p.2);
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