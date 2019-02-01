fn validate(n: u32) -> bool {
	// assert!(123456 <= n && n <= 987654);
	let digits: Vec<u32> = format!("{}", n).chars().map(|c| c as u32 - '0' as u32).collect();

	for (i, digit) in digits.iter().enumerate() {
		if *digit == 0 {
			return false;
		}
		for (j, other) in digits.iter().enumerate() {
			if i == j { continue }
			if digit == other {
				return false;
			}
		}

		if n % digit > 0 {
			return false;
		}
	}
	true
}

fn main() {
	for i in 123456..=987654 {
		if validate(i) {
			println!("{}", i);
		}
	}
}
