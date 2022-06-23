#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Pilha{
	data: Vec<usize>
}

impl Pilha {
	fn new() -> Pilha {
		Pilha {
			data: Vec::new()
		}
	}

	fn adicionar_elemento(&mut self, e: usize){
		self.data.push(e);
	}

	fn remover_elemento(&mut self) -> usize {
		match self.data.pop() {
			Some(x) => x,
			_ => 0
		}
	}

	fn tamanho_pilha(&self) -> usize {
		self.data.len()
	}

	fn inserir_posicao(&mut self, indice: usize, conteudo: usize){
		self.data.insert(indice, conteudo)
	}
}

type Input = usize;
type Output = (Vec<usize>, usize); 

fn main() {
	let mut itens: Vec<usize> = Vec::new();
	
    loop{
		let entrada: Input = inputline_usize();

		if entrada == 0 {
			break
		}else{
			itens.push(entrada);		
		}
	}

	for i in itens{
		let output: Output = problema(i);

		print!("Discarded cards: ");
		for j in 0..(output.0.len() - 1) {
			print!("{}, ", output.0[j]);
		}
		print!("{}", output.0[output.0.len() - 1]);
		print!("\nRemaining card: {}\n", output.1);
	}
}

fn problema(entrada: Input) -> Output {
	let mut pilha: Pilha = Pilha::new();
	let mut pilha_descarte: Pilha = Pilha::new();
	
	pilha.data = (1..=entrada).collect();
	pilha.data.reverse();

	loop {
		pilha_descarte.adicionar_elemento(pilha.remover_elemento());
		let mut item: usize = pilha.remover_elemento();
		pilha.inserir_posicao(0, item);

		if pilha.tamanho_pilha() <  2 {
			break;
		}
	}

	return (pilha_descarte.data, pilha.data[0]);
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
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
		let p: Output = problema(7);
		let r: Vec<Input> = Vec::from([
			1, 3, 5, 7, 4, 2
		]);

		assert_eq!(p, (r, 6));
	}

	#[test]
	fn test_2() {
		let p: Output = problema(19);
		let r: Vec<Input> = Vec::from([
			1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 4, 8, 12, 16, 2, 10, 18, 14
		]);

		assert_eq!(p, (r, 6));
	}

	#[test]
	fn test_3() {
		let p: Output = problema(10);
		let r: Vec<Input> = Vec::from([
			1, 3, 5, 7, 9, 2, 6, 10, 8
		]);

		assert_eq!(p, (r, 4));
	}

	#[test]
	fn test_4() {
		let p: Output = problema(6);
		let r: Vec<Input> = Vec::from([
			1, 3, 5, 2, 6
		]);

		assert_eq!(p, (r, 4));
	}
} 