use boards::hp35;

use arbitrary_int::u10;

fn main() {
  let mut board = hp35::Board::new(to_u10(include_str!("../../hp35/roms/35v4.obj")));
  let _ = simple_logger::init_with_level(log::Level::Trace);
  for _ in 0..260 {
    board.run_cycle(None);
  }
}

fn to_u10(contents: &str) -> Vec<u10> {
  contents
    .lines()
    .filter_map(|line| {
      Some(u10::new(u16::from_str_radix(line, 8).unwrap()))
    })
    .collect()
}