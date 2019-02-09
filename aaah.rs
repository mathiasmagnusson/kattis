use std::io::stdin;

fn main() {
	let mut buf = String::new();
	let willing = stdin().read_line(&mut buf).unwrap();
	let needed  = stdin().read_line(&mut buf).unwrap();
	println!("{}o", if willing >= needed {
		"g"
	} else {
		"n"
	});
}
