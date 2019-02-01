use std::io::stdin;

fn main() {
	let n = {
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();
		line.trim().parse().unwrap()
	};

	let mut l: Vec<u16> = (0..n).map(|_| { let mut line = String::new(); stdin().read_line(&mut line).unwrap(); line.trim().parse().unwrap() }).collect();

	let mut swaps = 0;
	for i in 0..n {
		for j in (0..i).rev() {
			if l[j] > l[j + 1] {
				swaps += 1;
				let tmp = l[j];
				l[j] = l[j + 1];
				l[j + 1] = tmp;
			}
		}
	}

	println!("{}", swaps);
}