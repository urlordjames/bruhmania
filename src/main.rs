extern crate reqwest;

mod map_result;
use map_result::MapResult;

use std::fs::File;
use std::io::prelude::*;

fn main() {
	// open maps.txt and put split it by line
	let mut file = match File::open("maps.txt") {
		Ok(file) => file,
		Err(error) => match error.kind() {
			std::io::ErrorKind::NotFound => panic!("maps.txt does not exist, you should create it"),
			_ => panic!("cannot open maps.txt: {}", error)
		}
	};
	let mut map_list = String::new();
	file.read_to_string(&mut map_list).unwrap();
	let maps: Vec<&str> = map_list.split_terminator("\n").collect();

	// create http client, this is reused for every request for performance
	let client = reqwest::blocking::Client::builder()
		.user_agent("trackmania leaderboards thing (https://github.com/urlordjames/bruhmania)")
		.build().unwrap();

	// TODO: multithreading
	let map_results: Vec<MapResult> = maps.iter().map(|map| MapResult::from_id(&client, map).unwrap()).collect();

	for map in map_results {
		println!("time is: {} uploaded at: {}", map.get_time(), map.get_timestamp());
	}
}
