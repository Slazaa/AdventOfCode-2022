use std::fs;
use std::ops::Range;

pub fn day4() {
	const INPUT_FILENAME: &str = "src/day4/input.txt";

	let input = match fs::read_to_string(INPUT_FILENAME) {
		Ok(x) => x.replace("\r", ""),
		Err(_) => {
			println!("Invalid filename: '{}'", INPUT_FILENAME);
			return;
		}
	};

	let count = input
		.split("\n")
		.collect::<Vec<&str>>()
		.iter()
		.map(|pair| {
			let ranges = pair
				.split(",")
				.collect::<Vec<&str>>()
				.iter()
				.map(|range| {
					let range = range
						.split("-")
						.collect::<Vec<&str>>()
						.iter()
						.map(|val| val.parse::<u32>().unwrap())
						.collect::<Vec<u32>>();

					range[0]..range[1]
				})
				.collect::<Vec<Range<u32>>>();

			let (fst, snd) = (&ranges[0], &ranges[1]);

			if 
				(fst.start <= snd.start && fst.end >= snd.start) || (fst.start <= snd.end && fst.end >= snd.end) ||
				(snd.start <= fst.start && snd.end >= fst.start) || (snd.start <= fst.end && snd.end >= fst.end)
			{
				1
			} else {
				0
			}
		})
		.collect::<Vec<u32>>()
		.iter()
		.sum::<u32>();

	println!("{}", count);
}