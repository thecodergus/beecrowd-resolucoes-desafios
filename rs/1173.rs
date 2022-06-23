fn main() {
    let entrada: i64 = inputline_i64();

	let mut result: Vec<i64> = vec![entrada; 10];

	for i in 0..10 {
		result[i] *=  u32::pow(2, i as u32) as i64;
	}

	for (index, value) in result.iter().enumerate() {
		println!("N[{}] = {}", index, value);
	}
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn string_to_i64(string: String) -> i64 {
	return string
			.trim()
			.parse::<i64>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_i64() -> i64 {
    return string_to_i64(inputline());
            
}