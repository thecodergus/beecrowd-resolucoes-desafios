use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(PartialEq, Clone, Debug)]
struct Ponto(u16, u16);

impl Ponto{
	fn calcular_distancia(&self, ponto: &Ponto) -> f64 {
		(((ponto.0 as f64) - (self.0 as f64)).powi(2) + ((ponto.1 as f64) - (self.1 as f64)).powi(2)).sqrt()
	}
}

#[derive(PartialEq, Clone, Debug)]
struct Retangulo{
	x: Ponto,
	y: Ponto
}

type Points = Vec<Ponto>;
	
impl Retangulo{
	fn new(linha: String) -> Retangulo {
		let (w, h, x0, y0): (u16, u16, u16, u16) = Retangulo::get_values(&linha);
		
		return Retangulo{
			x: Ponto(x0, x0 + w),
			y: Ponto(y0, y0 + h)
		};
	}

	fn get_values(linha: &String) -> (u16, u16, u16, u16){
		let linha_split: Vec<String> = linha
										.split_whitespace()
										.into_iter()
										.map(|x| x.to_string())
										.collect();

		let convert = |x: &String| x.trim().parse::<u16>().unwrap_or(0);

		return (
			convert(&linha_split[0]),
			convert(&linha_split[1]),
			convert(&linha_split[2]),
			convert(&linha_split[3])
		);
	}

	fn get_points(&self) -> Points {
		vec![
			Ponto(self.x.0, self.y.0), // A
			Ponto(self.x.1, self.y.1), // G
			Ponto(self.x.0, self.y.1), // E
			Ponto(self.x.1, self.y.0), // C
			Ponto(self.x.1 / 2, self.y.0), // B
			Ponto(self.x.1, self.y.1 / 2), // H
			Ponto(self.x.1 / 2, self.y.1 / 2), // F
			Ponto(self.x.0, self.y.1 / 2), // D
			Ponto(self.x.1 / 2, self.y.1 / 2) // G
		]
/*
		
						E--------F--------G
						|			  	  |
						|			  	  |
						D		 G  	  H
						|			  	  |
						|			  	  |
						A--------B--------C  	
*/
	}

	fn ponto_pertence(&self, ponto: &Ponto) -> bool {
		(self.x.0 <= ponto.0 && ponto.0 <= self.x.1)
		&&
		(self.y.0 <= ponto.1 && ponto.1 <= self.y.1) 

		// (self.x.0, ponto.0) <= (ponto.0, self.x.1)
		// &&
		// (self.y.0, ponto.1) <= (ponto.1, self.y.1)

		// (self.x.0, ponto.0, self.y.0, ponto.1) <= (ponto.0, self.x.1, ponto.1, self.y.1)

		// self.x.0 <= ponto.0 && ponto.0 <= self.x.1 && self.y.0 <= ponto.1 && ponto.1 <= self.y.1
	}
}

#[derive(PartialEq, Clone, Debug)]
struct Ataque {
	magia: String,
	nivel_magia: u8,
	centro_explosao: Ponto
}

impl Ataque{
	fn new(linha: String) -> Ataque {

		let (magia, nivel_magia, p1, p2): (String, u8, u16, u16) = Ataque::get_values(&linha); 

		return Ataque {
			magia,
			nivel_magia,
			centro_explosao: Ponto(
				p1,
				p2
			)
		}
	}

	fn get_values(linha: &String) -> (String, u8, u16, u16) {
		let linha_split: Vec<String> = linha
										.split_whitespace()
										.into_iter()
										.map(|x| x.to_string())
										.collect();
		
		let convert_u16 = |x: &String| x.parse::<u16>().unwrap_or(0);
		let convert_u8 = |x: &String| x.parse::<u8>().unwrap_or(0);

		return (
			linha_split[0].to_owned(),
			convert_u8(&linha_split[1]),
			convert_u16(&linha_split[2]),
			convert_u16(&linha_split[3])
		);
	}
}

fn main() {
    let num_entradas: u16 = inputline_u16();
	let mut result: Vec<u16> = Vec::new();

	for _ in 0..num_entradas{
		let retangulo: Retangulo = Retangulo::new(inputline());
		let ataque: Ataque = Ataque::new(inputline());
		result.push(problema(retangulo, ataque));
	}

	for i in result{
		println!("{}", i);
	}
}

fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn string_to_u16(string: String) -> u16 {
	return string
			.trim()
			.parse::<u16>()
			.expect("an error happed in convertion Stringo to Int64");
}

fn inputline_u16() -> u16 {
    return string_to_u16(inputline());
            
}

fn problema(retangulo: Retangulo, ataque: Ataque) -> u16 {
	// "Banco de Dados da aplicação"
	let mut tabela_habilidades: HashMap<&str, Vec<u16>> = HashMap::new();
	tabela_habilidades.insert("fire", Vec::from([200, 20, 30, 50]));
	tabela_habilidades.insert("water", Vec::from([300, 10, 25, 40]));
	tabela_habilidades.insert("earth", Vec::from([400, 25, 55, 70]));
	tabela_habilidades.insert("air", Vec::from([100, 18, 38, 60]));
	
	// get Infos from tabela
	let info_habilidade: &Vec<u16> = &tabela_habilidades
		.get(ataque.magia.as_str())
		.unwrap();

	let nivel_magia: usize = ataque.nivel_magia as usize;

	if retangulo.ponto_pertence(&ataque.centro_explosao){
		return return info_habilidade[0];
	}
	
	for i in (retangulo.x.0)..=(retangulo.x.1) {
		for j in (retangulo.y.0)..=(retangulo.y.1){
			if ataque.centro_explosao.calcular_distancia(&Ponto(i, j)) <= (info_habilidade[nivel_magia] as f64){
				return info_habilidade[0];
			}
		}
	}
	
	return 0;
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let p: u16 = problema(
			Retangulo::new("10 10 50 50".to_string()),
			Ataque::new("fire 1 85 55".to_string())
		);
		let r: u16 = 0;

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {

		let p: u16 = problema(
			Retangulo::new("10 10 50 50".to_string()),
			Ataque::new("fire 2 85 55".to_string())
		);
		let r: u16 = 200;

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let p: u16 = problema(
			Retangulo::new("10 10 50 100".to_string()),
			Ataque::new("water 3 100 100".to_string())
		);
		let r: u16 = 300;

		assert_eq!(p, r);
	}

	#[test]
	fn test_4() {

		let p: u16 = problema(
			Retangulo::new("10 10 50 100".to_string()),
			Ataque::new("air 3 100 100".to_string())
		);
		let r: u16 = 100;

		assert_eq!(p, r);
	}

	#[test]
	fn test_5() {

		let p: u16 = problema(
			Retangulo::new("41 17 34 100".to_string()),
			Ataque::new("earth 1 124 119".to_string())
		);
		let r: u16 = 0;

		assert_eq!(p, r);
	}

	#[test]
	fn test_6() {

		let p: u16 = problema(
			Retangulo::new("78 108 112 14".to_string()),
			Ataque::new("earth 1 95 5".to_string())
		);
		let r: u16 = 400;

		assert_eq!(p, r);
	}

	#[test]
	fn test_7() {

		let p: u16 = problema(
			Retangulo::new("78 108 112 14".to_string()),
			Ataque::new("fire 1 95 5".to_string())
		);
		let r: u16 = 200;

		assert_eq!(p, r);
	}

	#[test]
	fn test_8() {

		let p: u16 = problema(
			Retangulo::new("27 36 141 54".to_string()),
			Ataque::new("earth 1 3 2".to_string())
		);
		let r: u16 = 0;

		assert_eq!(p, r);
	}

	#[test]
	fn test_9() {

		let p: u16 = problema(
			Retangulo::new("53 118 47 44".to_string()),
			Ataque::new("earth 1 57 112".to_string())
		);
		let r: u16 = 400;

		assert_eq!(p, r);
	}

	#[test]
	fn test_10() {

		let p: u16 = problema(
			Retangulo::new("144 39 26 73".to_string()),
			Ataque::new("earth 1 88 137".to_string())
		);
		let r: u16 = 400;

		assert_eq!(p, r);
	}

	#[test]
	fn test_11() {

		let p: u16 = problema(
			Retangulo::new("105 124 131 52".to_string()),
			Ataque::new("earth 1 100 50".to_string())
		);
		let r: u16 = 0;

		assert_eq!(p, r);
	}

	#[test]
	fn test_12() {
		let p: u16 = problema(
			Retangulo::new("95 59 109 8".to_string()),
			Ataque::new("earth 1 138 71".to_string())
		);
		let r: u16 = 400;

		assert_eq!(p, r);
	}
} 