fn main() {
	problema(inputline_u64());
}

fn problema(n: u64) {
	for i in (1..=(n * 4)).step_by(4){
		for j in i..(i + 3) {
			print!("{} ", j);
		}
		println!("PUM");
	}
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
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