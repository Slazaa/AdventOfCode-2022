use std::fs;

pub fn day1() {
	const INPUT_FILENAME: &str = "src/day1/input.txt";

	let input = match fs::read_to_string(INPUT_FILENAME) {
		Ok(x) => x.replace("\r", ""),
		Err(_) => {
			println!("Invalid filename: '{}'", INPUT_FILENAME);
			return;
		}
	};

	let mut elves_calories = input
		.split("\n\n")
		.collect::<Vec<&str>>()
		.iter()
		.map(|elf|
			elf
				.split("\n")
				.collect::<Vec<&str>>()
				.iter()
				.map(|cal| cal.parse::<u32>().unwrap())
				.collect::<Vec<u32>>()
				.iter()
				.sum()
		)
		.collect::<Vec<u32>>();

	elves_calories.sort();
	elves_calories.reverse();

	let tree_max_calories = elves_calories[..3]
		.iter()
		.sum::<u32>();

	println!("{:?}", tree_max_calories);
}