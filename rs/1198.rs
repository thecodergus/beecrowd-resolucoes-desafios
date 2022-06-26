fn main() {
	let result: Vec<u64> = eof();
	
    for i in result{
		println!("{}", i);
	}
}

fn problema(a: i64, b: i64) -> u64 {
	(b - a).abs() as u64
}

fn eof() -> Vec<u64> {
	let mut result: Vec<u64> = Vec::new();
	let mut buffer = String::new();

	let mut bytes: usize;

	loop {
		bytes = std::io::stdin()
			.read_line(&mut buffer)
			.unwrap();

		if bytes == 0 {
	        break;
	    }

		let entrada: Vec<i64> = split_to_vec_i64(buffer.to_string().replace("\n", ""));
		
		result.push(problema(entrada[1], entrada[0]));

		buffer = String::from("");
	}

	return result;
}

fn split_to_vec_i64(vetor: String) -> Vec<i64> {
	vetor
		.split_whitespace()
		.into_iter()
		.map(|x| x.trim().parse::<i64>().unwrap_or(0))
		.collect::<Vec<i64>>()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_1() {
		let p = problema(10, 12);
		let r = 2;

		assert_eq!(p, r);
	}

	#[test]
	fn test_2() {
		let p = problema(10, 14);
		let r = 4;

		assert_eq!(p, r);
	}

	#[test]
	fn test_3() {
		let p = problema(100, 200);
		let r = 100;

		assert_eq!(p, r);
	}

	
	
} 