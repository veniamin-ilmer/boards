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
  pub fn new(binary0: Vec<u8>, binary1: Vec<u8>, binary2: Vec<u8>, binary3: Vec<u8>, binary4: Vec<u8>) -> Self {
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
    self.i4003s[0].read_write_serial(shifter::Direction::Left, ports & 0b10 == 0, ports & 0b1 == 0);
    
    //Shifter 1 = Printer
    let out = self.i4003s[1].read_write_serial(shifter::Direction::Left, ports & 0b10 == 0b10, ports & 0b100 == 0);
    //Shifter 2 = Cascade shifter 1, for Printer
    self.i4003s[2].read_write_serial(shifter::Direction::Left, out, ports & 0b100 == 0);
  }
  
  pub fn printer_shift_bits(&self) -> u32 {
    let shift1 = self.i4003s[1].read_parallel() as u32;
    let shift2 = self.i4003s[2].read_parallel() as u32;
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