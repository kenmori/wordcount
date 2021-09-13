use regex::Regex;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn count(input: impl BufRead) -> HashMap<String, usize> {
  let re = Regex::new(r"\w+").unwrap();
  let mut freqs = HashMap::new(); // HashMap<String, usize>型

  for line in input.lines() {
    let line = line.unwrap();
    for m in re.find_iter(&line) {
      let word = m.as_str().to_string();
      *freqs.entry(word).or_insert(0) += 1;
    }
  }
  freqs
}

pub fn check_above3(freqs: &HashMap<String, usize>) -> bool {
  let mut is_include_avobe3 = false;
  for (_key, value) in freqs.iter() {
    if *value > 2 {
      is_include_avobe3 = true
    }
  }
  return is_include_avobe3;
}

fn main() {
  let filename = env::args().nth(1).expect("1 argument FILENAME required");
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(&file);

  let freqs = count(reader);
  let is_above3 = check_above3(&freqs);
  println!("{:?}, {}", freqs, is_above3);
}
