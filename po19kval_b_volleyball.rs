use std::io::stdin;

fn main() {
	let nootin = {
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();
		line.split("")
			.filter_map(|c| {
				if c == "A" {
					Some(true)
				} else {
					if c == "B" {
						Some(false)
					} else {
						None
					}
				}
			}).collect::<Vec<bool>>()
	};

	let mut a_sets = 0_u32;
	let mut b_sets = 0_u32;
	let mut balls_processed = 0_usize;

	for set_max in &[25, 25, 15] {
		let mut a_score = 0_u32;
		let mut b_score = 0_u32;
		for ball in &nootin[balls_processed..] {
			balls_processed += 1;

			if *ball {
				a_score += 1;
			} else {
				b_score += 1;
			}

			if a_score >= *set_max && a_score > b_score + 1 {
				a_sets += 1;
				break;
			}

			if b_score >= *set_max && b_score > a_score + 1 {
				b_sets += 1;
				break;
			}
		}
	}

	println!("{} {}", a_sets, b_sets);
}
