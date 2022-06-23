fn main() {
	let mut entrada: Vec<usize> = Vec::new();
	
	loop {
		let input = inputline_usize();
		
		if input == 0 {
			break;
		}

		entrada.push(input);
	}

	for i in entrada {
		println!("{}", problema(i));
	}
}

fn problema(n: usize) -> usize {
	for i in 1..=300 {
		if my_solution(n, i) == 13 {
			return i;
		}
	}

	return 0;
}

fn josephus(n: usize, k: usize) -> usize {
	match n {
		1 => 1,
		_ => (josephus(n - 1, k) + (k - 1)) % n + 1
	}
}

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

// Minha solucao
fn my_solution(n: usize, k: usize) -> usize {
	if n == 1{
		return 1;
	}
	
	let mut pessoas: Vec<Pessoa> = vec![Pessoa::new(); n as usize];
	let verificar = |x: &Vec<Pessoa>| x.iter()
										.filter(|y| y.viva)
										.collect::<Vec<&Pessoa>>()
										.len() as u32;

	let mut proxima_morte: usize = 0;
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

#[test]
fn caso_1() {
	assert_eq!(problema(17), 7);
}

#[test]
fn caso_2() {
	assert_eq!(problema(13), 1);
}

#[test]
fn caso_3() {
	assert_eq!(problema(14), 18);
}

#[test]
fn caso_4() {
	assert_eq!(problema(15), 10);
}

#[test]
fn caso_5() {
	assert_eq!(problema(16), 11);
}

#[test]
fn caso_6() {
	assert_eq!(problema(18), 17);
}

#[test]
fn caso_7() {
	assert_eq!(problema(19), 11);
}

#[test]
fn caso_8() {
	assert_eq!(problema(20), 15);
}

#[test]
fn caso_9() {
	assert_eq!(problema(21), 29);
}

#[test]
fn caso_10() {
	assert_eq!(problema(22), 5);
}

#[test]
fn caso_11() {
	assert_eq!(problema(23), 21);
}

#[test]
fn caso_12() {
	assert_eq!(problema(24), 13);
}

#[test]
fn caso_13() {
	assert_eq!(problema(25), 26);
}

#[test]
fn caso_14() {
	assert_eq!(problema(26), 14);
}

#[test]
fn caso_15() {
	assert_eq!(problema(83), 137);
}