use std::io::stdin;

fn main() {
	let mut line = String::new();

	let (r, c) = {
		stdin().read_line(&mut line).unwrap();
		let line: Vec<u32> = line.trim().split(' ').map(|n| n.parse().unwrap()).collect();
		(line[0], line[1])
	};

	type Cell = Option<u8>;
	type Grid = Vec<Vec<Cell>>;
	let grid: Grid = (0..r).map(|_| {
		line.clear();
		stdin().read_line(&mut line).unwrap();
		line.split(' ').map(|s| s.bytes().next().unwrap()).map(|ch| {
			if ch == b'-' {
				None
			} else {
				Some(ch - b'0')
			}
		}).collect()
	}).collect();

	type Region = Vec<(u8, u8)>;
	let regions: Vec<Region> = {
		line.clear();
		stdin().read_line(&mut line).unwrap();
		let region_count: usize = line.trim().parse().unwrap();

		(0..region_count).map(|_| {
			line.clear();
			stdin().read_line(&mut line).unwrap();
			let mut line = line.trim().split(' ');
			let cell_count: usize = line.next().unwrap().parse().unwrap();
			line.map(|cell| {
				let mut cell = cell.bytes();
				cell.next().unwrap();
				let x = cell.next().unwrap() - b'0';
				cell.next().unwrap();
				let y = cell.next().unwrap() - b'0';
				(x, y)
			}).collect()
		}).collect()
	};

	loop {
		for row in &grid {
			for cell in row {
				if cell.is_some() {
					continue;
				}
				
			}
		}
	}
}