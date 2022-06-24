
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Tempo(u8, u8);

impl Tempo{
	fn new(hora: u8, minuto: u8) -> Tempo {
		Tempo(hora, minuto)
	}
	
	fn add_minuto(&mut self) -> &Tempo {
		if self.1 < 59 {
			self.1 += 1;
		}else{
			self.1 = 0;
			self.add_hora();
		}

		return self;
	}

	fn add_hora(&mut self) -> &Tempo {
		if self.0 < 23{
			self.0 += 1;
		}else{
			self.0 = 0;
		}
		

		return self;
	}

	fn equals(&self, outro: &Tempo) -> bool {
		(self.0, self.1) == (outro.0, outro.1)
	}
}

fn main() {
	let result: Tempo = problema(inputline());
    println!("O JOGO DUROU {} HORA(S) E {} MINUTO(S)", result.0, result.1);
}

fn problema(linha: String) -> Tempo {
	let entrada: Vec<u8> = linha
							.split_whitespace()
							.into_iter()
							.map(|x| x.trim().parse::<u8>().unwrap())
							.collect();

	let (mut inicio, fim): (Tempo, Tempo) = (Tempo::new(entrada[0], entrada[1]), Tempo::new(entrada[2], entrada[3]));
	let mut contador: Tempo = Tempo(0, 0);

	// while inicio != fim {
	while inicio != fim {
		inicio.add_minuto();
		
		contador.add_minuto();
	}

	if Tempo(0, 0) == contador {
		return Tempo(24, 0);
	}
	
	return contador;
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let p = problema("7 8 9 10".to_string());
		let r = Tempo(2, 2);

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let p = problema("7 7 7 7".to_string());
		let r = Tempo(24, 0);

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let p = problema("7 10 8 9".to_string());
		let r = Tempo(0, 59);

		assert_eq!(p, r);
	}

	#[test]
	fn test_4() {
		let p = problema("10 12 10 11".to_string());
		let r = Tempo(23, 59);

		assert_eq!(p, r);
	}

	#[test]
	fn test_5() {
		let p = problema("1 1 1 1".to_string());
		let r = Tempo(24, 0);

		assert_eq!(p, r);
	}
	
} 