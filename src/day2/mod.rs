use std::fs;

pub fn day2() {
	const INPUT_FILENAME: &str = "src/day2/input.txt";

	let input = match fs::read_to_string(INPUT_FILENAME) {
		Ok(x) => x.replace("\r", ""),
		Err(_) => {
			println!("Invalid filename: '{}'", INPUT_FILENAME);
			return;
		}
	};

	let rounds = input
		.split("\n")
		.collect::<Vec<&str>>()
		.iter()
		.map(|round| {
			let round = round
				.split(" ")
				.collect::<Vec<&str>>();

			(
				round[0].chars().nth(0).unwrap(),
				round[1].chars().nth(0).unwrap()
			)
		})
		.collect::<Vec<(char, char)>>();

	let round_scores = rounds
		.iter()
		.map(|round| {
			let mut round = *round;

			round.1 = if round.1 == 'X' {
				if round.0 == 'A' { 'Z' }
				else if round.0 == 'B' { 'X' }
				else { 'Y' }
			} else if round.1 == 'Y' {
				if round.0 == 'A' { 'X' }
				else if round.0 == 'B' { 'Y' }
				else { 'Z' }
			} else {
				if round.0 == 'A' { 'Y' }
				else if round.0 == 'B' { 'Z' }
				else { 'X' }
			};

			let mut res = if 
				(round.0 == 'A' && round.1 == 'Y') ||
				(round.0 == 'B' && round.1 == 'Z') ||
				(round.0 == 'C' && round.1 == 'X')
			{
				6
			} else if
				(round.0 == 'A' && round.1 == 'X') ||
				(round.0 == 'B' && round.1 == 'Y') ||
				(round.0 == 'C' && round.1 == 'Z')
			{
				3
			} else {
				0
			};
			

			res += match round.1 {
				'X' => 1,
				'Y' => 2,
				'Z' => 3,
				_ => panic!("Invalid letter: '{}'", round.1)
			};

			res
		})
		.collect::<Vec<u32>>();

	let score = round_scores
		.iter()
		.sum::<u32>();

	println!("{}", score);
}