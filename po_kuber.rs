use std::io::stdin;

fn main() {
	println!("{}", (0..={
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();
		line.trim().parse().unwrap()
	}).map(|n: usize| n.pow(3)).sum::<usize>());
}