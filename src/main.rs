use std::env;

mod day1;
mod day2;
mod day3;

fn main() {
	let args = env::args()
		.skip(1)
		.collect::<Vec<String>>();

	match args[0].as_str() {
		"day1" => day1::day1(),
		"day2" => day2::day2(),
		"day3" => day3::day3(),
		_ => println!("Invalid name: '{}'", args[0])
	}
}