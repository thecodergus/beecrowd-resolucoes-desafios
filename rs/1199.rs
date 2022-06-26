fn main() {
    let mut result: Vec<String> = Vec::new();
	
	loop {
		let entrada: String = inputline();

		if entrada.contains("-") {
			break;
		}

		result.push(problema(entrada));
	}

	for i in result{
		println!("{}", i);
	}
}

fn problema(entrada: String) -> String {
	match entrada.contains("0x") {
		true => to_integer(entrada),
		false => to_hex(entrada)
	}
}

fn to_integer(hex: String) -> String {
	u32::from_str_radix(hex.trim_start_matches("0x"), 16)
		.unwrap_or(0)
		.to_string()
}

fn to_hex(int: String) -> String {
	format!("0x{:X}", int.parse::<u32>().unwrap())
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let p = problema("4".to_string());
		let r = "0x4".to_string();

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let p = problema("7".to_string());
		let r = "0x7".to_string();

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let p = problema("44".to_string());
		let r = "0x2C".to_string();

		assert_eq!(p, r);
	}

	#[test]
	fn test_4() {
		let p = problema("0x80685".to_string());
		let r = "525957".to_string();

		assert_eq!(p, r);
	}
	
} 