extern crate reqwest;
extern crate json;
extern crate chrono;

use std::fmt;

#[derive(Debug)]
pub enum MapError {
	TimeNaN,
	DateNotString,
	CannotParseDate(chrono::format::ParseError)
}

impl fmt::Display for MapError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			MapError::TimeNaN => write!(f, "time is not a number"),
			MapError::DateNotString => write!(f, "date is not a string"),
			MapError::CannotParseDate(e) => write!(f, "cannot parse date: {}", e)
		}
	}
}

impl std::error::Error for MapError {
}

pub struct MapResult {
	time: json::number::Number,
	timestamp: chrono::DateTime::<chrono::FixedOffset>
}

impl MapResult {
	pub fn from_id(client: &reqwest::blocking::Client, map_id: &str) -> Result<MapResult, MapError> {
		let url = format!("https://trackmania.io/api/leaderboard/map/{}", map_id);

		let resp = client.get(url).send().unwrap().text().unwrap();

		let json = json::parse(&resp).unwrap();

		let top = &json["tops"][0];

		Ok(MapResult {
			time: match top["time"].as_number() {
				None => return Err(MapError::TimeNaN),
				Some(n) => (f64::from(n) / 1000.0).into()
			},
			timestamp: match top["timestamp"].as_str() {
				None => return Err(MapError::DateNotString),
				Some(d) => match chrono::DateTime::parse_from_rfc3339(d) {
					Err(e) => return Err(MapError::CannotParseDate(e)),
					Ok(parsed) => parsed
				}
			}
		})
	}

	pub fn get_time(&self) -> json::number::Number {
		self.time
	}

	pub fn get_timestamp(&self) -> chrono::DateTime::<chrono::FixedOffset> {
		self.timestamp
	}
}
