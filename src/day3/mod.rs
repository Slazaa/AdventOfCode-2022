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
		.iter()
		.map(|rucksack| {
			let (fst_comp, snd_comp) = rucksack
				.split_at(rucksack.len() / 2);

			println!("{} - {}", fst_comp, snd_comp);

			for i in fst_comp.chars() {
				for j in snd_comp.chars() {
					if i == j {
						if i.is_lowercase() {
							return (i as u8 - 'a' as u8 + 1) as u32;
						} else {
							return (i as u8 - 'A' as u8 + 27)  as u32;
						}
					}
				}
			}

			0
		})
		.collect::<Vec<u32>>()
		.iter()
		.sum::<u32>();

	println!("Sum: {}", sum);
}