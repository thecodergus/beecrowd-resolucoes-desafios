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

	fn get_minutes(&self) -> u32 {
		((self.0 as u32) * 60) + (self.1 as u32)
	}
}

fn main() {
	let mut result: Vec<u32> = vec![];
	
    loop {
		let entrada: Vec<u8> = inputline_vector_u8();

		if (entrada[0], entrada[1], entrada[2], entrada[3]) == (0,0,0,0) {
			break;
		}

		result.push(problema(&entrada));
	}

	for i in result{
		println!("{}", i);
	}
}

fn problema(entrada: &Vec<u8> ) -> u32 {
	let (mut A, B, mut aux): (Tempo, Tempo, Tempo) = (Tempo::new(entrada[0], entrada[1]), Tempo::new(entrada[2], entrada[3]), Tempo(0, 0));

	if A == B {
		return 0;
	}

	while A != B {
		A.add_minuto();
		aux.add_minuto();
	}
	
	return aux.get_minutes();
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

fn convert_vector_string_to_u8(vetor: Vec<String>) -> Vec<u8> {
	let mut result: Vec<u8> = Vec::new();

	for i in vetor {
		result.push(i.to_string().parse::<u8>().unwrap());
	}

	return result;
}

fn inputline_vector_u8() -> Vec<u8> {
	return convert_vector_string_to_u8(splitline(inputline()));
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let p: u32 = problema(&vec![1, 5, 3, 5]);
		let r: u32 = 120;

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let p: u32 = problema(&vec![23, 59, 0, 34]);
		let r: u32 = 35;

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let p: u32 = problema(&vec![21, 33, 21, 10]);
		let r: u32 = 1417;

		assert_eq!(p, r);
	}
	
} 