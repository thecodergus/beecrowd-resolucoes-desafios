#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Numero(u32, u32);

trait Planejamento {
	fn new(int: u32, num: u32) -> Numero;
}

impl Planejamento for Numero {
	fn new(int: u32, num: u32) -> Numero {
		Numero(int, num)
	}
}

type Input = Vec<u32>;
type Output = Vec<Numero>;

fn main() {
    let num_vezes: u32 = inputline_u32(); 
	let mut entrada: Input = Vec::new();

	for _ in 0..num_vezes {
		entrada.push(inputline_u32());
	}

	let maior_numero: u32 = maior_num(&entrada);

	for i in problema(entrada, maior_numero){
		println!("{} aparece {} vez(es)", i.0, i.1)
	}
}

fn problema(numeros: Input, maior_num: u32) -> Output {
	let mut result: Output = Vec::new();
	let mut aux: Vec<u32> = vec![0; maior_num as usize + 1];

	for i in numeros {
		aux[i as usize ] += 1;
	}

	for (index, value) in aux.iter().enumerate() {
		if *value > 0 {
			result.push(Numero((index) as u32, *value));
		}
	}
	
	return result;
}

fn maior_num(entrada: &Input) -> u32 {
	let mut maior = entrada[0];

	for i in 1..entrada.len() {
		if entrada[i] > maior {
			maior = entrada[i];
		}
	}

	return maior;
}

#[test]
fn caso_1() {
	assert_eq!(problema(
		vec![
			8,
			10,
			8,
			260,
			4,
			10,
			10
		]
	), 
			   Vec::from([
				   Numero(4, 1),
				   Numero(8, 2),
				   Numero(10, 3),
				   Numero(260, 1)
			   ]));
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn string_to_u32(string: String) -> u32 {
	return string
			.trim()
			.parse::<u32>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_u32() -> u32 {
    return string_to_u32(inputline());
            
}