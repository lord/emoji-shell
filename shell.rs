use std::io::timer;
use std::io::stdio;

fn main() {
  stdio::print("Loading...");
  stdio::flush();
  timer::sleep(1000);
  for num in range(0, 1000) {
    stdio::print("\r\x1b[K" + num.to_str());
    stdio::flush();
    timer::sleep(100);
  }
}