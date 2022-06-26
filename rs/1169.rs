fn main() {
    let casos: u128 = inputline_u128();
	let mut result: Vec<u128> = Vec::new();

	for _ in 0..casos {
		result.push(problema(inputline_u8()));
	}

	for i in result {
		println!("{} kg", i);
	}
	
}

fn problema(num_casas: u8) -> u128 {
	let mut graos: u128 = 1;

	for _ in 0..num_casas {
		graos *= 2;
	}

	// 12(12 graos = 1 grama) * 1000 (mil gramas = 1 kg)
	return graos / 12000;
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn string_to_u128(string: String) -> u128 {
	return string
			.trim()
			.parse::<u128>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_u128() -> u128 {
    return string_to_u128(inputline());
            
}

fn string_to_u8(string: String) -> u8 {
	return string
			.trim()
			.parse::<u8>()
			.unwrap_or(0);
}

fn inputline_u8() -> u8 {
    return string_to_u8(inputline());
            
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let p: u128 = problema(7);
		let r: u128 = 0;

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let p: u128 = problema(19);
		let r: u128 = 43;

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let p: u128 = problema(14);
		let r: u128 = 1;

		assert_eq!(p, r);
	}
} 