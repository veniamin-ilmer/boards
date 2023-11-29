use boards::fairchild_ves;

fn main() {
  let mut board = fairchild_ves::Board::new(include_bytes!("../../fairchild_ves/roms/SL31253.bin").clone(),
                                            include_bytes!("../../fairchild_ves/roms/SL31254.bin").clone(),
                                            Some(include_bytes!("../../fairchild_ves/roms/a.out").to_vec()));
  let _ = simple_logger::init_with_level(log::Level::Trace);
  for _ in 0..100000 {
//    if board.roms[0].pc0 == 0x008F || board.roms[0].pc0 == 0x0091 || board.roms[0].pc0 == 0x0092 || board.roms[0].pc0 == 0x0093 || board.roms[0].pc0 == 0x0095 || board.roms[0].pc0 == 0x0096 {
//      start = true;
//    } else if start {
    board.roms[0].print();
    board.cpu.print();
    board.ports[0] |= 0b00001111;  //Clear out console buttons
    board.run_cycle();
  }
}