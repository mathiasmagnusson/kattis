fn main() {
	use std::io::Read;
	let mut line = String::new();
	std::io::stdin().read_to_string(&mut line).unwrap();

	println!("{}", if line.contains("ss") { "hiss" } else { "no hiss" });
}