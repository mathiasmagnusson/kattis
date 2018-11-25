use std::io::{stdin};

fn main() {
	for i in 1..=({
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();
		line.trim().parse::<u32>().unwrap()
	}) {
		translate_alien_shit(i);
	}
}

fn translate_alien_shit(case_number: u32) {
	let mut line = String::new();
	stdin().read_line(&mut line).unwrap();
	let args = line.trim().split(" ").collect::<Vec<&str>>();
	let alien_number = args[0];
	let source_language = args[1];
	let target_language = args[2];

	let alien_base = source_language.len();

	let mut alien_number_int = 0u128;
	for (i, c) in alien_number.chars().rev().enumerate() {
		let character_value = source_language.find(c).unwrap() as u128 * alien_base.pow(i as u32) as u128;
		alien_number_int += character_value;
	}

	let target_base = target_language.len();

	let mut target_number = Vec::new();
	for i in 0.. {
		if alien_number_int / target_base.pow(i) as u128 == 0 {
			break;
		}
		let character_value = (alien_number_int % target_base.pow(i + 1) as u128) / target_base.pow(i) as u128;
		target_number.push(target_language.chars().nth(character_value as usize).unwrap());
	}

	print!("Case #{}: ", case_number);

	for c in target_number.iter().rev() {
		print!("{}", c);
	}

	println!("\n");
}

