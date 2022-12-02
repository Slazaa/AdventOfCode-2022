use std::env;

mod day1;
mod day2;

fn main() {
	let args = env::args()
		.skip(1)
		.collect::<Vec<String>>();

	match args[0].as_str() {
		"day1" => day1::day1(),
		"day2" => day2::day2(),
		_ => println!("Invalid name: '{}'", args[0])
	}
}