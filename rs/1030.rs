#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Pessoa{
	viva: bool
}

trait Suicida {
	fn new() -> Pessoa;
	fn morreu(&mut self);
}

impl Suicida for Pessoa{
	fn new() -> Pessoa {
		Pessoa{
			viva: true
		}
	}

	fn morreu(&mut self) {
		self.viva = false
	}
}

fn main() {
    let num_casos: usize = inputline_usize();
	let mut aux: Vec<usize>;
	let mut result: Vec<usize> = Vec::new();
	
	for _ in 0..num_casos{
		aux = inputline_vector_usize();

		result.push(problema(aux[0], aux[1]));
	}

	for (index, value) in result.iter().enumerate() {
		println!("Case {}: {}", index + 1, value);
	}
}
// Solucao comum

fn problema(n: usize, k: usize) -> usize {
	match n {
		1 => 1,
		_ => (problema(n - 1, k) + (k - 1)) % n + 1
	}
}

// Minha solucao
fn problema2(n: usize, k: usize) -> usize {
	if n == 1 || n <= k{
		return 1;
	}
	
	let mut pessoas: Vec<Pessoa> = vec![Pessoa::new(); n as usize];
	let verificar = |x: &Vec<Pessoa>| x.iter()
										.filter(|y| y.viva)
										.collect::<Vec<&Pessoa>>()
										.len() as u32;

	let mut proxima_morte: usize = k - 1;
	let mut contador: usize = 0;
	
	loop {
		if contador == proxima_morte {
			pessoas[contador].morreu();

			if verificar(&pessoas) == 1 {
				break;
			}else{
				// Calcular proximo a se matar

				let mut pessoa_que_morreu: usize = proxima_morte;
				let mut aux: usize = 0;

				loop {
					pessoa_que_morreu += 1;

					if pessoa_que_morreu == n {
						pessoa_que_morreu = 0;
					}

					if pessoas[pessoa_que_morreu].viva {
						aux += 1
					}

					if aux == k {
						proxima_morte = pessoa_que_morreu;
						break;
					}
				}
			}
		}

		contador += 1;
		if contador == n {
			contador = 0;
		}
	}
	
	let mut fico_vivo: usize = 0;

	for (index, value) in pessoas.iter().enumerate() {
		if value.viva {
			fico_vivo = index + 1;
		}
	}
	
	return fico_vivo;
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

fn splitline(string: String) -> Vec<String> {
	let itens: Vec<&str> = string.split_whitespace().collect();

	let mut result: Vec<String> = Vec::new();

	for i in itens {
		result.push(i.to_string());
	}
	
	return result;
}

fn convert_vector_string_to_usize(vetor: Vec<String>) -> Vec<usize> {
	let mut result: Vec<usize> = Vec::new();

	for i in vetor {
		result.push(i.to_string().parse::<usize>().unwrap());
	}

	return result;
}

fn inputline_vector_usize() -> Vec<usize> {
	return convert_vector_string_to_usize(splitline(inputline()));
}


#[test]
fn caso_1() {
	assert_eq!(problema(5, 2), 3);
}

#[test]
fn caso_2() {
	assert_eq!(problema(6, 3), 1);
}

#[test]
fn caso_3() {
	assert_eq!(problema(1234, 233), 25);
}

#[test]
fn caso_4() {
	assert_eq!(problema(10, 3), 4);
}

#[test]
fn caso_5() {
	assert_eq!(problema(1057, 21), 514);
}

#[test]
fn caso_6() {
	assert_eq!(problema(1, 1), 1);
}

#[test]
fn caso_7() {
	assert_eq!(problema(10000 , 1000), 5981);
}

#[test]
fn caso_8() {
	assert_eq!(problema(1, 1000), 1);
}

#[test]
fn caso_9() {
	assert_eq!(problema(10000, 1), 10000);
}

#[test]
fn caso_10() {
	assert_eq!(problema(10000, 2), 3617);
}

#[test]
fn caso_11() {
	assert_eq!(problema(10000, 3), 2692);
}

#[test]
fn caso_12() {
	assert_eq!(problema(10, 3), 4);
}

#[test]
fn caso_13() {
	assert_eq!(problema(13, 2), 11);
}

#[test]
fn caso_14() {
	assert_eq!(problema(5049, 938), 1026);
}