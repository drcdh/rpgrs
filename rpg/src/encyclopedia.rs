use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::effect::Effect;

#[derive(Serialize, Deserialize, Debug)]
struct Encyclopedia<T: fmt::Display + serde::Serialize> {
    listing: Vec::<T>,
}

impl<T: fmt::Display + serde::Serialize> fmt::Display for Encyclopedia<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.listing).ok().unwrap())
    }
}

pub type Effects = Encyclopedia<Effect>;

pub fn get_effects_encyclopedia(filename: &str) -> Result<Effects, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let een = serde_json::from_reader(reader)?;
    Ok(een)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effects_test() {
        let filename = "data/effects.json";
        let een = get_effects_encyclopedia(filename).expect("Could not parse encyclopedia file");
        println!(">>> EFFECTS <<<\n{}", een);
    }
}
