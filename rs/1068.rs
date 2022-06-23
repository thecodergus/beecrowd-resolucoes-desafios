fn main() {
    let entrada: Vec<String> = input_eof_to_vector_string();
	let mut result: Vec<bool> = Vec::new();
	
	for i in entrada {
		result.push(problema(i.to_owned()));
	}

	for i in result{
		if i {
			println!("correct");
		}else{
			println!("incorrect");
		}
	}
}

fn problema(caso: String) -> bool {
	let vetor: Vec<char> = caso.chars().collect();
	let (mut esquerdo, mut direito): (usize, usize) = (0, 0);
	let mut aberto = false;
	
	for i in 0..vetor.len() {
		if vetor[i] == '(' {
		
			esquerdo += 1;
			aberto = true;
		}
		if vetor[i] == ')' {
			if !aberto && esquerdo == direito {
				return false;
			}
			
			direito += 1;
			aberto = false
		}
	}

	// println!("{:?}", vetor);
	
	return esquerdo == direito && !aberto;
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

fn input_eof_to_vector_string() -> Vec<String> {
    return eof_to_vector_string();
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let p = problema(String::from("a+(b*c)-2-a"));
		let r = true;

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let p = problema(String::from("(a+b*(2-c)-2+a)*2"));
		let r = true;

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let p = problema(String::from("(a*b-(2+c)"));
		let r = false;

		assert_eq!(p, r);
	}

	#[test]
	fn test_4() {
		let p = problema(String::from("2*(3-a))"));
		let r = false;

		assert_eq!(p, r);
	}

	#[test]
	fn test_5() {
		let p = problema(String::from(")3+b*(2-c)("));
		let r = false;

		assert_eq!(p, r);
	}

	#[test]
	fn test_6() {
		let p = problema(String::from("("));
		let r = false;

		assert_eq!(p, r);
	}

	#[test]
	fn test_7() {
		let p = problema(String::from(")"));
		let r = false;

		assert_eq!(p, r);
	}

	#[test]
	fn test_8() {
		let p = problema(String::from("()"));
		let r = true;

		assert_eq!(p, r);
	}

	#[test]
	fn test_9() {
		let p = problema(String::from("(()())"));
		let r = true;

		assert_eq!(p, r);
	}

	#[test]
	fn test_10() {
		let p = problema(String::from("(x+y))((x-y)"));
		let r = false;

		assert_eq!(p, r);
	}


	#[test]
	fn caso_11() {
		let p = problema(String::from(")(()"));
		let r = false;

		assert_eq!(p, r);
	}

	#[test]
	fn caso_15() {
		let p = problema(String::from(")(()"));
		let r = false;

		assert_eq!(p, r);
	}
	
} 