type Linha = Vec<String>;

fn main() {
	let mut resultado: Vec<u64> = Vec::new();
	
    loop {
		let input: Linha = splitline(inputline());

		if input[0] == "0" && input[1] == "0" {
			break;
		}

		resultado.push(problema(input));
	}

	for i in resultado {
		if i == 0 {
			println!("No carry operation.");
		}else if i == 1{
			println!("{} carry operation.", i);
		}else{
			println!("{} carry operations.", i);
		}
		
		
	}
}

fn problema(entrada: Linha) -> u64 {
	let (a, b): (String, String) = (entrada[0].to_owned(), entrada[1].to_owned());
	let mut a_chars: Vec<u32> = a.chars()
									.collect::<Vec<char>>()
									.iter()
									.map(|x| string_to_u32(x.to_string()))
									.collect::<Vec<u32>>();
	let mut b_chars: Vec<u32> = b.chars()
									.collect::<Vec<char>>()
									.iter()
									.map(|x| string_to_u32(x.to_string()))
									.collect::<Vec<u32>>();

	if a_chars.len() > b_chars.len() {
		std::mem::swap(&mut a_chars, &mut b_chars);
	}
	
	return problema_aux(a_chars.to_owned(), b_chars.to_owned());
	// return 0;
}


fn problema_aux(mut a: Vec<u32>, mut b: Vec<u32>) -> u64 {
	
	if b.len() == 0 {
		return 0;
	}	
	
	let aa = match a.pop() {
		Some(x) => x,
		None => 0
	};

	let bb = match b.pop() {
		Some(x) => x,
		None => 1
	};	

	// Visualização melho
	let conta: u32 = aa + bb;
	
	if conta >= 10 {
		let onde: isize = (b.len() as isize) - 1;

		if onde >= 0 {
			b[onde as usize] += 1;
		}

		return 1 + problema_aux(a, b);
	}else{
		return 0 + problema_aux(a, b);
	}
}
 
fn string_to_u32(string: String) -> u32 {
	return string
			.trim()
			.parse::<u32>()
			.expect("an error happed in convertion Stringo to Int32");
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let p = problema(Vec::from([
			"123".to_string(), 
			"456".to_string()]));
		let r = 0;

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let p = problema(Vec::from(["555".to_string(), "555".to_string()]));
		let r = 3;

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let p = problema(Vec::from(["123".to_string(), "594".to_string()]));
		let r = 1;

		assert_eq!(p, r);
	}

	#[test]
	fn test_4() {
		let p = problema(Vec::from(["146423".to_string(), "446456".to_string()]));
		let r = 1;

		assert_eq!(p, r);
	}

	#[test]
	fn test_5() {
		let p = problema(Vec::from(["545655".to_string(), "55578".to_string()]));
		let r = 5;

		assert_eq!(p, r);
	}

	#[test]
	fn test_6() {
		let p = problema(Vec::from(["146423".to_string(), "594644".to_string()]));
		let r = 3;

		assert_eq!(p, r);
	}

	#[test]
	fn test_7() {
		let p = problema(Vec::from(["454".to_string(), "989765".to_string()]));
		let r = 3;

		assert_eq!(p, r);
	}
} 