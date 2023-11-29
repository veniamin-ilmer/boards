use chips::{rom,ram,shifter,cpu,cpu::i4004};
use log::warn;
use arbitrary_int::{u2,u4};

pub struct Board {
  pub i4001s: [rom::I4001; 5],
  pub i4002s: [ram::I4002; 2],
  pub i4003s: [shifter::I4003; 3],
  pub i4004: cpu::I4004, 
  advance_paper: bool,
  hammering: bool,
}

impl Board {
  pub fn new() -> Self {
    let binary0 = vec![0xf0, 0x11, 0x01, 0x50, 0xb0, 0x51, 0x5f, 0xad, 0xb1, 0xf0, 0x51, 0x5f, 0xad, 0x1c, 0x29, 0x68,
                       0x51, 0x73, 0x27, 0xec, 0xf5, 0xb3, 0x68, 0xf0, 0x51, 0xa0, 0xf3, 0xb3, 0xf5, 0xe1, 0x66, 0x27,
                       0xea, 0xf5, 0xf7, 0x14, 0x00, 0x52, 0x46, 0x40, 0x00, 0xb0, 0xec, 0xf8, 0xf8, 0xe4, 0x27, 0xea,
                       0xe7, 0x50, 0x64, 0x27, 0xea, 0xe6, 0x34, 0x20, 0xa0, 0xa5, 0xb1, 0x30, 0x68, 0x51, 0x73, 0xd0,
                       0xe1, 0xd1, 0xf3, 0xf5, 0xfc, 0x85, 0x1a, 0x00, 0xf0, 0x00, 0x00, 0x11, 0x4f, 0x50, 0xb0, 0x26,
                       0x20, 0x28, 0x10, 0x53, 0x00, 0x51, 0x00, 0x71, 0x5a, 0x60, 0x14, 0x4b, 0xf7, 0x14, 0x57, 0x43,
                       0x02, 0xd4, 0x40, 0xd4, 0xd3, 0x29, 0xe2, 0xd0, 0xe2, 0xc0, 0x6c, 0x22, 0x20, 0x23, 0xea, 0xf6,
                       0x73, 0x6d, 0x1a, 0x76, 0xf0, 0xbc, 0xc0, 0xa9, 0x14, 0xd9, 0x28, 0x00, 0xf0, 0x51, 0x4a, 0x40,
                       0xf7, 0xbb, 0xc7, 0x63, 0x53, 0x19, 0x1a, 0x68, 0x58, 0x05, 0x41, 0x31, 0x18, 0x22, 0x12, 0x05,
                       0x0c, 0x9d, 0x6d, 0x3d, 0xbd, 0x8d, 0x5d, 0x2d, 0x06, 0x7d, 0x4d, 0x1d, 0x0d, 0xad, 0xa4, 0x0e,
                       0xbf, 0x06, 0x91, 0x98, 0xf1, 0xcd, 0xd7, 0xfd, 0x8a, 0x05, 0x61, 0xf9, 0xd7, 0xd7, 0xca, 0xc5,
                       0x50, 0x6a, 0x28, 0x07, 0x50, 0x64, 0x79, 0xb4, 0x26, 0x18, 0x22, 0x00, 0xd1, 0x50, 0x65, 0x27,
                       0xea, 0xfc, 0xb9, 0xa2, 0xf5, 0xf7, 0x1c, 0x77, 0xa9, 0x79, 0xcd, 0x40, 0x7a, 0x14, 0x61, 0xb2,
                       0xf5, 0xfa, 0xf6, 0xb2, 0x83, 0xb3, 0xd0, 0x82, 0xb2, 0x50, 0x64, 0x77, 0xbf, 0x29, 0xa2, 0xf5,
                       0xf7, 0x14, 0xf8, 0xef, 0xf2, 0xf7, 0x1c, 0xf7, 0xec, 0xb9, 0x29, 0xa3, 0xe0, 0x69, 0x29, 0xe9,
                       0x1c, 0x7a, 0xa2, 0xe0, 0x69, 0xa9, 0xe4, 0xdf, 0xe7, 0x28, 0x00, 0x26, 0x10, 0x19, 0xfd, 0xc0];

    let binary1 = vec![0x33, 0xa5, 0xf2, 0xf2, 0x86, 0xb8, 0xb6, 0x41, 0x0e, 0x66, 0x66, 0x66, 0x66, 0x66, 0x27, 0xe9,
                       0x29, 0xe0, 0x69, 0x77, 0x0e, 0x27, 0xec, 0xb3, 0xed, 0x29, 0xe5, 0xb3, 0xe4, 0xc0, 0xd4, 0x85,
                       0xb6, 0x29, 0xe9, 0x27, 0xeb, 0xfb, 0xe0, 0x69, 0x77, 0x21, 0xf1, 0xc0, 0xd4, 0x85, 0xb8, 0x41,
                       0x33, 0xd4, 0x85, 0xb6, 0xfa, 0xf9, 0x29, 0xe8, 0xf1, 0x27, 0xeb, 0xfb, 0xe0, 0x69, 0x77, 0x35,
                       0x1a, 0x43, 0x6d, 0xc1, 0x68, 0x68, 0x68, 0x68, 0x68, 0x68, 0x29, 0xe0, 0x79, 0x4a, 0xe4, 0xe5,
                       0xc0, 0x68, 0x68, 0x29, 0xe9, 0xbd, 0xe0, 0x79, 0x53, 0xc0, 0xde, 0xb9, 0xad, 0x68, 0x68, 0xbd,
                       0xa9, 0xf8, 0xf1, 0xb9, 0x29, 0xe9, 0xbd, 0xe0, 0xa9, 0x1c, 0x61, 0xc0, 0x68, 0x68, 0x29, 0xee,
                       0xf8, 0xf3, 0xc1, 0x68, 0x68, 0x68, 0x68, 0x29, 0xee, 0xf6, 0xc1, 0x27, 0xee, 0xf5, 0xc1, 0x66,
                       0x66, 0x66, 0x27, 0xe6, 0xc0, 0x66, 0x66, 0xd1, 0x41, 0x81, 0xd8, 0x41, 0x81, 0xa4, 0x41, 0x82,
                       0x27, 0xee, 0xf5, 0xfa, 0xf6, 0xe6, 0xc0, 0xd4, 0x85, 0xb8, 0xde, 0xb9, 0x41, 0xa2, 0xde, 0xb9,
                       0xbd, 0x68, 0x29, 0xdf, 0xeb, 0x79, 0xa2, 0xf3, 0xc1, 0xad, 0xfb, 0xc1, 0xad, 0xf8, 0xbd, 0xf3,
                       0xc1, 0xd7, 0x95, 0xc1, 0xdc, 0x84, 0xc1, 0xa5, 0xf6, 0xb5, 0xf3, 0xc1, 0xa4, 0xf6, 0xc1, 0xd4,
                       0x85, 0xb8, 0x29, 0xab, 0xe5, 0xc0, 0x29, 0xed, 0xbb, 0xc0, 0x7b, 0xcd, 0x6a, 0xc0, 0xdd, 0x9b,
                       0xf3, 0xd0, 0x9a, 0xc1, 0x42, 0xd3, 0x00, 0x42, 0x94, 0x42, 0xa3, 0x42, 0xaa, 0x42, 0xae, 0x42,
                       0xb3, 0x42, 0xb9, 0x42, 0xca, 0x42, 0xde, 0x42, 0xe7, 0x42, 0xec, 0x42, 0x46, 0x44, 0x00, 0x51,
                       0x80, 0x51, 0x81, 0x51, 0x81, 0x2a, 0x00, 0x40, 0x00, 0x6f, 0x6f, 0x6f, 0x6f, 0x6f, 0x6f, 0xbf];

    let binary2 = vec![0xf4, 0xbf, 0x7f, 0x10, 0xda, 0x51, 0x4a, 0x2e, 0xff, 0xba, 0xdf, 0xb9, 0x29, 0xe0, 0x42, 0x2c,
                       0x7f, 0x17, 0xdf, 0xbf, 0xa5, 0x42, 0x26, 0xd1, 0x8f, 0xf7, 0x14, 0x25, 0xa4, 0xbf, 0xbe, 0xf6,
                       0xf3, 0xde, 0xf6, 0x42, 0x26, 0xa4, 0xbe, 0x29, 0xed, 0xba, 0xed, 0xbb, 0x11, 0x2c, 0xd2, 0xbd,
                       0xec, 0xf6, 0xf7, 0xe1, 0x50, 0xb0, 0x68, 0x6b, 0xab, 0xb9, 0x51, 0xa2, 0xf7, 0x14, 0x37, 0x11,
                       0x3f, 0xf0, 0xe1, 0xe2, 0x7d, 0x53, 0x2a, 0x0c, 0x2e, 0x00, 0xd8, 0x11, 0x4b, 0xe1, 0x50, 0xb0,
                       0x7b, 0x4b, 0xc0, 0x50, 0x6a, 0xb8, 0xdd, 0x9f, 0xf1, 0x1c, 0x5f, 0xba, 0xdf, 0x42, 0x61, 0x27,
                       0xe9, 0x77, 0x77, 0xaa, 0x1c, 0x68, 0x52, 0x8f, 0xaf, 0x52, 0x8a, 0xae, 0x52, 0x8a, 0x19, 0x6e,
                       0xd2, 0x29, 0xe1, 0x50, 0xb2, 0x42, 0x3f, 0x52, 0x8a, 0xaa, 0x14, 0x83, 0x97, 0xf1, 0x1c, 0x83,
                       0xda, 0x52, 0x8a, 0xa7, 0x9b, 0xf7, 0x14, 0x56, 0x42, 0x5c, 0x9c, 0xf1, 0x1c, 0x8f, 0xfa, 0xd1,
                       0xf5, 0xf5, 0x40, 0x65, 0xa4, 0xbd, 0x26, 0x40, 0x27, 0xee, 0x1c, 0xa1, 0xd8, 0xe6, 0xf0, 0x51,
                       0x4a, 0x41, 0xc6, 0x51, 0x0a, 0x51, 0x46, 0x51, 0x49, 0xc0, 0xd3, 0xb5, 0xfa, 0xc1, 0xd1, 0xb3,
                       0xbb, 0x42, 0xc2, 0x52, 0xf9, 0xef, 0xe5, 0x42, 0xc2, 0x52, 0xf9, 0xdd, 0x9b, 0xf1, 0x82, 0xba,
                       0xf7, 0xba, 0x93, 0xbb, 0xf3, 0xba, 0x99, 0xba, 0xf1, 0xc0, 0x52, 0xf9, 0xa3, 0x8b, 0x82, 0xbb,
                       0xf7, 0xba, 0xc0, 0xd4, 0x85, 0xb6, 0x27, 0xec, 0xb2, 0x29, 0xec, 0x82, 0xf6, 0xc1, 0x52, 0xd6,
                       0xf7, 0x66, 0x27, 0xe4, 0xdf, 0xbd, 0xc0, 0x29, 0xec, 0xf4, 0xe4, 0xc0, 0xdb, 0x8d, 0x1a, 0xf1,
                       0x6e, 0xd0, 0x29, 0xeb, 0xfb, 0xe0, 0x79, 0xf1, 0xc0, 0x27, 0xed, 0xb2, 0x29, 0xed, 0xb3, 0xc0];

    let binary3 = vec![0x32, 0xc0, 0x30, 0x40, 0x4b, 0xed, 0x6c, 0x14, 0x75, 0x0e, 0xd9, 0xfc, 0xa7, 0x0f, 0xfb, 0x8d,
                       0x04, 0x02, 0x87, 0xef, 0xfc, 0x6d, 0x0f, 0x7b, 0x0f, 0x76, 0x46, 0x8d, 0xa2, 0x3c, 0x48, 0xa0,
                       0x73, 0xe1, 0x9e, 0x32, 0x9a, 0x36, 0xe5, 0x51, 0x51, 0x34, 0x29, 0x21, 0x51, 0xa9, 0x3f, 0x52,
                       0xa7, 0x29, 0x52, 0xca, 0xa7, 0x22, 0x53, 0xcf, 0x3c, 0xdd, 0xa7, 0x24, 0xff, 0x85, 0xf1, 0x5d,
                       0xce, 0x73, 0xdd, 0x5d, 0xa7, 0x40, 0x8d, 0x03, 0xe3, 0xe5, 0x0e, 0x49, 0x52, 0x5e, 0x5a, 0xa9,
                       0x56, 0xac, 0x4d, 0x21, 0xa7, 0x51, 0xa0, 0x40, 0xcf, 0x3c, 0x5e, 0x5a, 0xdd, 0xa7, 0x56, 0x86,
                       0xf3, 0xfe, 0xca, 0xca, 0xa7, 0x67, 0xfe, 0x7b, 0x6f, 0x90, 0x76, 0x47, 0x02, 0xa7, 0x1c, 0x04,
                       0x0c, 0xa7, 0x6a, 0x0d, 0xc2, 0xb1, 0x10, 0xb4, 0x7b, 0x6e, 0x9e, 0xdf, 0xcf, 0x9a, 0xce, 0x84,
                       0xca, 0x5f, 0xa7, 0x7c, 0xdd, 0x53, 0x9a, 0x7c, 0xa7, 0x3c, 0x6c, 0x66, 0x75, 0x66, 0xd9, 0xa7,
                       0x98, 0x6c, 0x98, 0x75, 0x97, 0xa7, 0x98, 0x82, 0xae, 0x7b, 0xb1, 0xaa, 0x77, 0xa3, 0xfd, 0xeb,
                       0xb4, 0xa8, 0xf1, 0xe9, 0x9a, 0x9e, 0xa7, 0x3c, 0xdb, 0xab, 0xfc, 0xc6, 0x03, 0xbc, 0xb0, 0xe7,
                       0xd4, 0xb7, 0x1e, 0x97, 0xbd, 0x31, 0x3c, 0x31, 0xbd, 0x1e, 0x2c, 0xbc, 0x01, 0x0d, 0xbf, 0xb4,
                       0xff, 0xb7, 0xac, 0x8a, 0xef, 0x82, 0x49, 0xd9, 0x4a, 0xfc, 0x4a, 0x7f, 0xf1, 0x6c, 0xd5, 0x75,
                       0xd3, 0x0b, 0x46, 0xfc, 0xef, 0xfd, 0xf1, 0xd7, 0xa9, 0xdf, 0x53, 0x9a, 0xe3, 0x5f, 0xf3, 0xbc,
                       0x5f, 0xe7, 0xf3, 0x74, 0xe8, 0xa7, 0xee, 0x00, 0xca, 0xce, 0xed, 0xdd, 0x5f, 0xc2, 0xb7, 0xda,
                       0xf3, 0xfd, 0x02, 0x0e, 0x03, 0x0c, 0x04, 0x0d, 0xf1, 0x09, 0xfa, 0x44, 0xf1, 0x09, 0xfa, 0xf1];

    let binary4 = vec![0x20, 0x28, 0x11, 0x06, 0x50, 0xb0, 0x26, 0x20, 0x28, 0x10, 0x32, 0xf0, 0x54, 0x50, 0x71, 0x11,
                       0x60, 0x14, 0x02, 0xf7, 0x14, 0x0e, 0x30, 0x44, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x51, 0xa7, 0x53, 0x61, 0x3e, 0x65, 0x63, 0x44,
                       0x9c, 0x5b, 0x55, 0x6a, 0x36, 0x58, 0x7a, 0x5d, 0x41, 0x5f, 0x85, 0x57, 0x98, 0x35, 0xa9, 0x5b,
                       0x9f, 0x7a, 0x96, 0x36, 0x59, 0x93, 0x2e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                       0x33, 0x41, 0xfe, 0x41, 0x48, 0x41, 0x4a, 0x68, 0x68, 0x41, 0x53, 0x41, 0x04, 0x41, 0x34, 0x41,
                       0x21, 0x41, 0xa2, 0x41, 0x9a, 0xbd, 0x29, 0xed, 0xbb, 0xc0, 0x2e, 0x6d, 0xab, 0xb7, 0xba, 0xf6,
                       0xab, 0xf6, 0x8e, 0xbb, 0xf7, 0xba, 0xb7, 0xf6, 0xf3, 0xc1, 0xaf, 0xb9, 0xfa, 0xd0, 0x29, 0xeb,
                       0xfb, 0xe0, 0x79, 0x7d, 0xc0, 0xaf, 0xb9, 0xf3, 0x29, 0xe9, 0x97, 0x12, 0x8e, 0xd9, 0xe0, 0x79,
                       0x87, 0xf0, 0xc0, 0x7b, 0x96, 0x6a, 0xfa, 0xc1, 0xaf, 0xf8, 0xbf, 0xc1, 0x41, 0x5f, 0x00, 0x27,
                       0xe6, 0x20, 0x40, 0x26, 0x00, 0x40, 0x4b, 0x41, 0x02, 0x41, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    Self {
      i4001s: [
                rom::I4001::new(binary0.try_into().unwrap()),
                rom::I4001::new(binary1.try_into().unwrap()),
                rom::I4001::new(binary2.try_into().unwrap()),
                rom::I4001::new(binary3.try_into().unwrap()),
                rom::I4001::new(binary4.try_into().unwrap())
              ],
      i4002s: [ram::I4002::new(), ram::I4002::new()],
      i4003s: [shifter::I4003::new(), shifter::I4003::new(), shifter::I4003::new()],
      i4004: cpu::I4004::new(),
      advance_paper: false,
      hammering: false,
    }
  }

  pub fn run_cycle(&mut self) {
    //Make Rust happy by borrowing things one at a time, then releasing them when done.
    {
      let mut i4004_io = I4004IO {
        i4001s: &mut self.i4001s,
        i4002s: &mut self.i4002s,
      };
      self.i4004.run_cycle(&mut i4004_io);
    }
    
    //ROM 0 has shifter data and clocks
    let ports = self.i4001s[0].read_ports().value();
    //Shifter 0 = Keyboard
    self.i4003s[0].read_write_serial(ports & 0b1 == 0, ports & 0b10 == 0);
    
    //Shifter 1 = Printer
    let out = self.i4003s[1].read_write_serial(ports & 0b100 == 0, ports & 0b10 == 0b10);
    //Shifter 2 = Cascade shifter 1, for Printer
    self.i4003s[2].read_write_serial(ports & 0b100 == 0, out);
  }
  
  pub fn printer_shift_bits(&self) -> usize {
    let shift1 = self.i4003s[1].read_parallel();
    let shift2 = self.i4003s[2].read_parallel();
    shift1 | (shift2 << 10)
  }
  
  pub fn new_advance_paper_signal(&mut self) -> bool {
    let ram0 = self.i4002s[0].read_ports().value();
    let advance_paper = ram0 & 0b1000 == 0b1000;
    if !self.advance_paper && advance_paper { //We only signal on the switch from false to true.
      self.advance_paper = true;
      return true;
    }
    self.advance_paper = advance_paper;
    false
  }
  
  ///Returns false if not hammering.
  ///Returns true if hammering.
  pub fn new_hammer_signal(&mut self) -> bool  {
    let ram0 = self.i4002s[0].read_ports().value();
    let hammering = ram0 & 0b10 == 0b10;
    if !self.hammering && hammering {
      self.hammering = true;
      return true;
    }
    self.hammering = hammering;
    false
  }
}


fn convert_ram_index(command_control: u4, designated_index: i4004::DesignatedIndex) -> usize {
  let bank = match command_control.value() {
    0b000 => 0,
    0b001 => 1,
    0b010 => 2,
    0b100 => 3,
    _ => { warn!("Invalid command control register: {}", command_control);
      0
    },
  };
  //bank takes upper 2 bits. designated index takes lower 2 bits.
  (designated_index.chip_index().value() | (bank << 2)) as usize
}

struct I4004IO<'a> {
  i4001s: &'a mut [rom::I4001; 5],
  i4002s: &'a mut [ram::I4002; 2],
}
impl i4004::IO for I4004IO<'_> {
  fn read_rom_byte(&self, address: i4004::ROMAddress) -> u8 {
    let high_addr = address.chip_index().value() as usize;
    let low_addr = address.offset();
    let i4001 = &self.i4001s[high_addr % self.i4001s.len()];  //Wrap around
    i4001.read(low_addr)
  }
  
  fn read_rom_ports(&self, designated_index: i4004::DesignatedIndex) -> u4 {
    let high_addr = (designated_index.chip_index() << 2 | designated_index.reg_index()).value() as usize;
    let i4001 = &self.i4001s[high_addr % self.i4001s.len()];  //Wrap around
    i4001.read_ports()
  }
  fn write_rom_ports(&mut self, designated_index: i4004::DesignatedIndex, value: u4) {
    let high_addr = (designated_index.chip_index() << 2 | designated_index.reg_index()).value() as usize;
    let i4001 = &mut self.i4001s[high_addr % self.i4001s.len()];  //Wrap around
    i4001.write_ports(value);
  }
  
  fn read_ram_character(&self, command_control: u4, designated_index: i4004::DesignatedIndex) -> u4 {
    let high_addr = convert_ram_index(command_control, designated_index);
    if let Some(i4002) = self.i4002s.get(high_addr) {
      i4002.read_character(designated_index.reg_index(), designated_index.char_index())
    } else {
      warn!("Write to nonexisting ram {}.", high_addr);
      u4::new(0)
    }
  }
  fn write_ram_character(&mut self, command_control: u4, designated_index: i4004::DesignatedIndex, value: u4) {
    let high_addr = convert_ram_index(command_control, designated_index);
    if let Some(i4002) = self.i4002s.get_mut(high_addr) {
      i4002.write_character(designated_index.reg_index(), designated_index.char_index(), value);
    } else {
      warn!("Write to nonexisting ram {}.", high_addr);
    }
  }
  fn read_ram_status(&self, command_control: u4, designated_index: i4004::DesignatedIndex, status_index: u2) -> u4 {
    let high_addr = convert_ram_index(command_control, designated_index);
    if let Some(i4002) = self.i4002s.get(high_addr) {
      i4002.read_status(designated_index.reg_index(), status_index)
    } else {
      warn!("Write to nonexisting ram {}.", high_addr);
      u4::new(0)
    }
  }
  fn write_ram_status(&mut self, command_control: u4, designated_index: i4004::DesignatedIndex, status_index: u2, value: u4) {
    let high_addr = convert_ram_index(command_control, designated_index);
    if let Some(i4002) = self.i4002s.get_mut(high_addr) {
      i4002.write_status(designated_index.reg_index(), status_index, value);
    } else {
      warn!("Write to nonexisting ram {}.", high_addr);
    }
  }
  fn write_ram_ports(&mut self, command_control: u4, designated_index: i4004::DesignatedIndex, value: u4) {
    let high_addr = convert_ram_index(command_control, designated_index);
    if let Some(i4002) = self.i4002s.get_mut(high_addr) {
      i4002.write_ports(value);
    } else {
      warn!("Write to nonexisting ram {}.", high_addr);
    }
  }
}