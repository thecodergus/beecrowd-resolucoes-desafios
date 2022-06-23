fn main() {
	println!("{}", problema(inputline_vector_u64()));
}

fn problema(entrada: Vec<u64>) -> u64 {
	let (a, b): (u64, u64) = (entrada[0], entrada[1]);
	let soma = |x: u64| (x*(x + 1)) / 2;
	
	return a + soma(b) - soma(a);
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

fn convert_vector_string_to_u64(vetor: Vec<String>) -> Vec<u64> {
	let mut result: Vec<u64> = Vec::new();

	for i in vetor {
		result.push(i.to_string().parse::<u64>().unwrap());
	}

	return result;
}

fn inputline_vector_u64() -> Vec<u64> {
	return convert_vector_string_to_u64(splitline(inputline()));
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let i: Vec<u64> = Vec::from([1, 5]);
		let p: u64 = problema(i);
		let r: u64 = 15;

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let i: Vec<u64> = Vec::from([1, 1000]);
		let p: u64 = problema(i);
		let r: u64 = 500500;

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let i: Vec<u64> = Vec::from([10, 20]);
		let p: u64 = problema(i);
		let r: u64 = 165;

		assert_eq!(p, r);
	}

	#[test]
	fn test_4() {
		let i: Vec<u64> = Vec::from([1, 1000000000]);
		let p: u64 = problema(i);
		let r: u64 = 500000000500000000;

		assert_eq!(p, r);
	}
} 