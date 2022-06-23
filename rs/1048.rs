type Reajuste = (f64, f64, i64);

fn main() {
    let entrada: f64 = inputline_f64();

	let result: Reajuste = problema(entrada);

	println!("Novo salario: {:.2}\nReajuste ganho: {:.2}\nEm percentual: {} %", result.0, result.1, result.2);
}

fn problema(valor: f64) -> Reajuste {
	let mut reajuste: i64 = 0;
	
	if valor >= 0.0 && valor <= 400.0 {
		reajuste = 15;
	}else if valor > 400.0 && valor <= 800.0 {
		reajuste = 12;
	}else if valor > 800.0 && valor <= 1200.0 {
		reajuste = 10;
	}else if valor > 1200.0 && valor <= 2000.0 {
		reajuste = 7;
	}else{
		reajuste = 4;
	}

	
	return (
		valor + (valor * ((reajuste as f64) / 100.0)),
		valor * ((reajuste as f64) / 100.0),
		reajuste
	);
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn string_to_f64(string: String) -> f64 {
	return string
			.trim()
			.parse::<f64>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_f64() -> f64 {
    return string_to_f64(inputline());
}

#[test]
fn caso_1() {
	assert_eq!(problema(400.0), (460.0, 60.0, 15));
}

#[test]
fn caso_2() {
	assert_eq!(problema(800.01), (880.01, 80.00, 10));
}

#[test]
fn caso_3() {
	assert_eq!(problema(2000.00), (2140.00, 140.00, 7));
}

#[test]
fn caso_4() {
	assert_eq!(problema(8965.19), (9323.80, 358.61, 4));
}

#[test]
fn caso_5() {
	assert_eq!(problema(0.00), (0.00, 0.00, 15));
}