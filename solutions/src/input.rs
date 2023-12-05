use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

pub fn file_input(filepath: &str) -> Vec<String> {
	let file = File::open(filepath).unwrap();
	let reader = BufReader::new(file);
	let mut res: Vec<String> = vec!();

	for line in reader.lines() {
		res.push(line.unwrap());
	}
	res
}