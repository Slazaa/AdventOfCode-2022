use std::fs;

pub fn day3() {
	const INPUT_FILENAME: &str = "src/day3/input.txt";

	let input = match fs::read_to_string(INPUT_FILENAME) {
		Ok(x) => x.replace("\r", ""),
		Err(_) => {
			println!("Invalid filename: '{}'", INPUT_FILENAME);
			return;
		}
	};

	let sum = input
		.split("\n")
		.collect::<Vec<&str>>()
		.chunks(3)
		.collect::<Vec<&[&str]>>()
		.iter()
		.map(|group| {
			let mut badge = ' ';

			for i in group[0].chars() {
				if group[1].contains(i) && group[2].contains(i) {
					badge = i;
					break;
				}
			}

			if badge == ' ' {
				panic!("One group has no badge :(");
			}

			if badge.is_lowercase() {
				(badge as u8 - 'a' as u8 + 1) as u32
			} else {
				(badge as u8 - 'A' as u8 + 27)  as u32
			}
		})
		.collect::<Vec<u32>>()
		.iter()
		.sum::<u32>();

	println!("Sum: {}", sum);
}