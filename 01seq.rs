use std::io::{self, BufRead};

#[derive(PartialEq, Clone, Copy)]
enum Char {
	Zero,
	One,
	Quest,
}

impl Char {
	fn from_int(int: u32) -> Char {
		if int == 0 {
			Char::Zero
		} else {
			Char::One
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let line: Vec<Char> = stdin.lock().lines().next().unwrap().unwrap().chars().map(|c| match c { '0' => Char::Zero, '1' => Char::One, '?' => Char::Quest, _ => panic!("") }).collect();

	let mut inversions = 0;
	for_every_possibility(&line, |seq: &[Char]| {
		let mut finishes = 0;

		for i in 0..seq.len() {
			if seq[i] == Char::Zero {
				inversions += i - finishes;
				finishes += 1;
			}
		}
	});

	println!("{}", inversions % 1_000_000_007);
}

fn for_every_possibility<F>(line: &[Char], mut callback: F)
	where F: FnMut(&[Char]) -> () {
	
	let mut q_indices = vec![];
	for (i, c) in line.iter().enumerate() {
		if *c == Char::Quest {
			q_indices.push(i);
		}
	}

	if q_indices.len() == 0 {
		callback(&line.to_vec());
		return;
	}

	// let mut s: Vec<Char> = vec![Char::Zero; q_indices.len()];
	// 'outer: loop {
	// 	let mut seq: Vec<Char> = line.to_vec();
	// 	for j in 0..s.len() {
	// 		seq[q_indices[j]] = s[j];
	// 	}

	// 	callback(&seq);

	// 	'inner: for i in (0..s.len()).rev() {
	// 		if s[i] == Char::Zero {
	// 			s[i] = Char::One;
	// 			break;
	// 		} else {
	// 			s[i] = Char::Zero;
	// 			if i == 0 {
	// 				break 'outer;
	// 			}
	// 		}
	// 	}
	// }

	for i in 0..2_u32.pow(q_indices.len() as u32) {
		let mut seq: Vec<Char> = line.to_vec();
		for j in 0..q_indices.len() {
			seq[q_indices[j]] = Char::from_int((1 << j) & i);
		}

		callback(&seq);
	}
}
