#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Ladrao{
	sitios_atacadas: Sitios
}

impl Ladrao{
	fn new() -> Ladrao {
		Ladrao{
			sitios_atacadas: Vec::new()
		}
	}

	fn roubou_ovelha(&mut self, sitio: &i64) -> &Ladrao {
		self.adicionar_sitio(sitio);
		return self;
	}

	fn adicionar_sitio(&mut self, sitio: &i64) {
		if !(self.sitios_atacadas.contains(sitio)){
			self.sitios_atacadas.push(sitio.to_owned());
		}
	}
}

type Sitios = Vec<i64>;

fn main() {
    let num_sitios: u64 = inputline_u64();
	let mut sitios: Sitios = inputline_vector_i64();
	let result: (u64, u64) = problema(&num_sitios, &mut sitios);

	println!("{} {}", result.0, result.1); 	
}

fn problema(num_sitios: &u64, sitios: &mut Sitios) -> (u64, u64) {
	let mut contador: i64 = 0;
	let mut p: bool = false;
	let mut irmao: Ladrao = Ladrao::new();
	let mut num_sitios_vizitados: u64 = 0;
	

	loop{
		if contador <= 0 && p || *num_sitios <= contador as u64 {
			break
		}

		let aux: i64 = sitios[contador as usize].to_owned();

		if sitios[contador as usize] > 0 {		
			sitios[contador as usize] -= 1;
			// irmao.roubou_ovelha(&contador);
			if num_sitios_vizitados <= contador as u64 {
				num_sitios_vizitados += 1;
			}
		}

		if eh_impar(aux) {
			contador += 1;
		}else{
			contador -= 1;
		}
		
	}

	// return (
	// 		irmao.sitios_atacadas.len() as u64, 
	// 		(*sitios).iter().sum::<i64>() as u64
	// 	);

	return (
			num_sitios_vizitados as u64, 
			(*sitios).iter().sum::<i64>() as u64
		);
}

fn eh_impar(num: i64) -> bool {
    (num & 1) == 1
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
		result.push(i.to_string().trim().parse::<i64>().unwrap());
	}

	return result;
}

fn inputline_vector_i64() -> Vec<i64> {
	return convert_vector_string_to_i64(splitline(inputline()));
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let mut arr: Vec<i64> = Vec::from([1, 3, 5, 7, 11, 13, 17, 19]);
		
		let p = problema(&8, &mut arr);
		let r: (u64, u64) = (8, 68);

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let mut arr: Vec<i64> = Vec::from([1, 3, 5, 7, 11, 13, 16, 19]);
		
		let p = problema(&8, &mut arr);
		let r: (u64, u64) = (7, 63);

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let mut arr: Vec<i64> = Vec::from([3, 3, 2, 3, 3]);
		
		let p = problema(&5, &mut arr);
		let r: (u64, u64) = (3, 9);

		assert_eq!(p, r);
	}

} 