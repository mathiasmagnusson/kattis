use std::io::stdin;

fn main() {
	let (ma, mc, _me) = {
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();
		let grades: Vec<u32> = line.trim().split(' ').map(|s| s.parse().unwrap()).collect();
		(grades[0], grades[1], grades[2])
	};

	let (ga, gc, _ge) = {
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();
		let grades: Vec<u32> = line.trim().split(' ').map(|s| s.parse().unwrap()).collect();
		(grades[0], grades[1], grades[2])
	};

	if ga == ma && gc == mc {
		println!("A");
		return;
	}

	if (ga > ma / 2 || ga == ma / 2 && ma % 2 == 0) && gc == mc {
		println!("B");
		return;
	}

	if gc == mc {
		println!("C");
		return;
	}

	if gc > mc / 2 || gc == mc / 2 && mc % 2 == 0 {
		println!("D");
		return;
	}

	println!("E");
}
