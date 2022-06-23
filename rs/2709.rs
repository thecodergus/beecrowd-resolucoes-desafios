fn main() {
    let entrada: Vec<bool> = eof();

	for i in entrada {
		if i {
			println!("You’re a coastal aircraft, Robbie, a large silver aircraft.");
		}else{
			println!("Bad boy! I’ll hit you.");
		}
	}
}

fn problema(num_moedas: usize, mut moedas: Vec<u64>, pulo: usize) -> bool {
	let mut result: u64 = 0;
	
	for i in 0..num_moedas{
		let aux: isize = ((num_moedas as isize) - 1) - ((pulo as isize) * (i as isize));

		if (aux as usize) >= num_moedas{
			break;
		}
		
		if aux >= 0 {
			result += moedas[aux as usize];
		}
	}
	
	return validar_numero_primo_u64(result);
}

fn validar_numero_primo_u64(n: u64) -> bool {
	if n <= 1 {
		return false;
	}

	for i in 2..=(((n as f64).sqrt()) as u64) {
		if n % i == 0 {
			return false;
		}
	}

	return true;
}

fn eof() -> Vec<bool> {
	let mut result: Vec<bool> = Vec::new();
	let mut buffer = String::new();

	let mut bytes: usize;

	loop {
		bytes = std::io::stdin()
			.read_line(&mut buffer)
			.unwrap();

		if bytes == 0 {
	        break;
	    }

		let num_moedas: usize = string_to_u64(buffer.to_string().trim().replace("\n", "")) as usize;
		let mut moedas: Vec<u64> = Vec::new();
		for _ in 0..num_moedas {
			moedas.push(string_to_u64(inputline()));
		}
		let pulo: usize = string_to_u64(inputline()) as usize;

		result.push(
			problema(num_moedas, moedas, pulo)
		);

		buffer = String::from("");
	}

	return result;
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().trim().replace("\n", "");
}

fn string_to_u64(string: String) -> u64 {
	return string
			.trim()
			.parse::<u64>()
			.expect("an error happed in convertion Stringo to Int64");
}
 

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let moedas: Vec<u64> = Vec::from([1, 2, 3, 4, 5]);
		let pulo: usize = 2;
		
		let p: bool = problema(moedas.len(), moedas, pulo);
		let r: bool = false;

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let moedas: Vec<u64> = Vec::from([1, 2, 3, 4, 5]);
		let pulo: usize = 3;
		
		let p: bool = problema(moedas.len(), moedas, pulo);
		let r: bool = true;

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let moedas: Vec<u64> = Vec::from([
			77,
			287,
			331,
			496,
			475,
			382,
			335
		]);
		let pulo: usize = 4;
		
		let p: bool = problema(moedas.len(), moedas, pulo);
		let r: bool = false;

		assert_eq!(p, r);
	}

	#[test]
	fn test_4() {
		let moedas: Vec<u64> = Vec::from([
			1,
			1
		]);
		let pulo: usize = 2;
		
		let p: bool = problema(moedas.len(), moedas, pulo);
		let r: bool = false;

		assert_eq!(p, r);
	}
} 