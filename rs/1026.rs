fn main() {
    let entrada: Vec<(u32, u32)> = input_eof_to_vector_u32();
	let mut resultado: Vec<u32> = Vec::new();

	for i in entrada {
		resultado.push(problema(i.0, i.1));
	}

	for i in resultado {
		println!("{}", i);
	}
}

fn problema(a: u32, b: u32) -> u32 {
	let mut result: Vec<char> = vec!['0'; 33];

	let aa: Vec<char> = to_u32_string(a, 32).chars().collect();
	let bb: Vec<char> = to_u32_string(b, 32).chars().collect();

	// println!("A: {:?}", aa);
	// println!("B: {:?}", bb);

	for i in 0..=32 {
		if (aa[i], bb[i]) == ('1', '0') || (aa[i], bb[i]) == ('0', '1') {
			result[i] = '1';
		}
	}

	// println!("R: {:?}", result);
	return to_string_u32(result);
}

fn to_u32_string(n: u32, tamanho_bit: u32) -> String {

	if n == 0 && tamanho_bit == 0{
		return 0.to_string();
	}
	
	if n == 1 && tamanho_bit == 0{
		return 1.to_string();
	}

	if n % 2 == 0 {
		return format!("{}0", to_u32_string(n / 2, tamanho_bit - 1));
	}

	return format!("{}1", to_u32_string(n / 2, tamanho_bit - 1));
	
}

fn to_string_u32(n: Vec<char>) -> u32 {
 	let mut cont: u32 = 0;
	
	for (i, value) in n.iter().rev().enumerate() {
		if *value == '1' {
			cont += u32::pow(2, i as u32);
		}
	}

	return cont;
}

fn eof_to_vector_string() -> Vec<String> {
	let mut result: Vec<String> = Vec::new();
	let mut buffer = String::new();

	let mut bytes: usize;

	loop {
		bytes = std::io::stdin()
			.read_line(&mut buffer)
			.unwrap();

		if bytes == 0 {
	        break;
	    }

		result.push(buffer.to_string().replace("\n", ""));

		buffer = String::from("");
	}

	return result;
}

fn convert_vector_string_to_u32(vetor: Vec<String>) -> Vec<(u32, u32)> {
	let mut result: Vec<(u32, u32)> = Vec::new();

	for i in vetor {
		let aux: Vec<String> = splitline(i.to_owned());
		result.push((aux[0].trim().parse::<u32>().unwrap(), aux[1].trim().parse::<u32>().unwrap()));
	}

	return result;
}

fn input_eof_to_vector_u32() -> Vec<(u32, u32)> {
    return convert_vector_string_to_u32(eof_to_vector_string());
}

fn splitline(string: String) -> Vec<String> {
	let itens: Vec<&str> = string.split_whitespace().collect();

	let mut result: Vec<String> = Vec::new();

	for i in itens {
		result.push(i.to_string());
	}
	
	return result;
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn conversao() {
		let r = 50345;

		assert_eq!(r, to_string_u32(to_u32_string(r, 32).chars().collect::<Vec<char>>()));
	}

	#[test]
	fn test_1() {
		let p = problema(4, 6);
		let r = 2;

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let p = problema(6, 9);
		let r = 15;

		assert_eq!(p, r);
	}
} 