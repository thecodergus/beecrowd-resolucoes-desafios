use std::collections::HashMap;

fn main() {
	let entrada: u8 = inputline_u8();

	println!("{}", problema(&entrada));
}

fn problema(n: &u8) -> String {
	let mut meses: HashMap<u8, String> = HashMap::new();
	
	meses.insert(1, "January".to_string());
	meses.insert(2, "February".to_string());
	meses.insert(3, "March".to_string());
	meses.insert(4, "April".to_string());
	meses.insert(5, "May".to_string());
	meses.insert(6, "June".to_string());
	meses.insert(7, "July".to_string());
	meses.insert(8, "August".to_string());
	meses.insert(9, "September".to_string());
	meses.insert(10, "October".to_string());
	meses.insert(11, "November".to_string());
	meses.insert(12, "December".to_string());

	return meses.get(n).unwrap().clone();
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn string_to_u8(string: String) -> u8 {
	return string
			.trim()
			.parse::<u8>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_u8() -> u8 {
    return string_to_u8(inputline());
            
}