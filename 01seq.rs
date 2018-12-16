use std::io::{self, BufRead};

#[derive(PartialEq, Clone, Copy)]
enum Char {
	Zero,
	One,
	Quest,
}

fn main() {
	let stdin = io::stdin();
	let line: Vec<Char> = stdin.lock().lines().next().unwrap().unwrap().chars().map(|c| match c { '0' => Char::Zero, '1' => Char::One, '?' => Char::Quest, _ => panic!("not okay man") }).collect();

	let mut inversions = 0_usize;
	for_every_possibility(&line, |seq: &[Char]| {
		let mut finishes = 0;

		for i in 0..seq.len() {
			if seq[i] == Char::Zero {
				inversions += i - finishes;
				finishes += 1;
				inversions %= 1_000_000_007;
			}
		}
	});

	println!("{}", inversions);
}

fn for_every_possibility<F>(line: &[Char], mut callback: F)
	where F: FnMut(&[Char]) -> () {
	
	// Find the indices of all question marks
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

	// the bits of `i` is all the arrangements that the questionmarks can be in
	for i in 0..2_u32.pow(q_indices.len() as u32) {
		// copy, since we'll write of the question marks
		let mut seq: Vec<Char> = line.to_vec();
		for j in 0..q_indices.len() {
			// place zeroes or ones in place of the questionmark
			seq[q_indices[j]] = if (1 << j) & i == 0 { Char::Zero } else { Char::One };
		}

		callback(&seq);
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
}
