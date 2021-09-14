use regex::Regex;

use std::collections::HashMap;
use std::io::BufRead;

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
