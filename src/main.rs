use std::env;
use std::fs::File;
use std::io::BufReader;

use wordcountlib::check_above3;
use wordcountlib::count;

fn main() {
  let filename = env::args().nth(1).expect("1 argument FILENAME required");
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(&file);

  let freqs = count(reader);
  let is_above3 = check_above3(&freqs);
  println!("{:?}, {}", freqs, is_above3);
}
