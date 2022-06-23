fn main() {
    for (i, j) in (1..=60).step_by(3).zip((0..=60).rev().step_by(5)) {
		println!("I={} J={}", i, j);
	}
}