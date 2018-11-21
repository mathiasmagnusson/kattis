use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let line = stdin.lock().lines().next().unwrap().unwrap();

	let mut inversions = 0;
	for_every_possibility(&line, |mut seq| {
		loop {
			let mut swapped = false;
			for i in 0..(seq.len() - 1) {
				if seq[i] == '1' && seq[i + 1] == '0' {
					seq[i    ] = '0';
					seq[i + 1] = '1';
					inversions = (inversions + 1) % 1_000_000_007;
					swapped = true;
				}
			}
			if !swapped { break; }
		}
	});

	println!("{}", inversions);
}

// Improve this:
fn for_every_possibility<F>(line: &str, mut callback: F)
	where F: FnMut(Vec<char>) -> () {
	
	let mut q_indices = vec![];
	for (i, c) in line.chars().enumerate() {
		if c == '?' {
			q_indices.push(i);
		}
	}

	if q_indices.len() == 0 {
		callback(line.split("").filter_map(|s| s.chars().next()).collect());
		return;
	}

	for i in 0..2u32.pow(q_indices.len() as u32) {
		let mut s: Vec<char> = format!("{:b}", i).split("").filter_map(|s| s.chars().next()).collect();
		while s.len() < q_indices.len() {
			s.insert(0, '0');
		}
		let mut seq: Vec<char> = line.split("").filter_map(|s| s.chars().next()).collect();
		for j in 0..s.len() {
			seq[q_indices[j]] = s[j];
		}

		callback(seq);
	}
}
