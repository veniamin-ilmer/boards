use std::io::prelude::*;
use std::fs::File;

use log::warn;
use chips::{rom,ram,cpu};

pub struct Board {
  rom: [rom::TMS2716; 4],
  ram: [ram::I2107B; 16],
  cpu: cpu::I8080,
}

fn load_rom(file_name: &str) -> rom::TMS2716 {
  let mut f = File::open(file_name).unwrap();
  let mut rom = Vec::new();
  f.read_to_end(&mut rom).unwrap();
  rom::TMS2716::new(rom.try_into().unwrap())
}

impl Board {
  pub fn new() -> Self {
    Self {
      rom: [load_rom("roms/invaders.h"),
            load_rom("roms/invaders.g"),
            load_rom("roms/invaders.f"),
            load_rom("roms/invaders.e")],
      ram: [ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new(), ram::I2107B::new()],
      cpu: cpu::I8080::new(),
    }
  }

  pub fn run_cycle(&mut self) {
    let mut cpu = cpu::I8080::new();  //TODO - Check if we should be using self.cpu instead
    
    let mut io = IO {
      rom: &mut self.rom,
      ram: &mut self.ram,
    };
    cpu.run_cycle(&mut io);
  }
}

struct IO<'a> {
  rom: &'a mut [rom::TMS2716; 4],
  ram: &'a mut [ram::I2107B; 16],
}

impl cpu::i8080::IO for IO<'_> {
  fn output(&mut self, port: u8, value: u8) {
    panic!("OUT {} {}", port, value);
  }
  
  fn input(&mut self, port: u8) -> u8 {
    panic!("IN {}", port);
  }
}

impl cpu::MemoryIO<u16> for IO<'_> {
  fn read_mem<T: chips::ReadArr>(&self, address: u16) -> T {
    match address {
      0..=0x1FFF => self.rom[address as usize / rom::TMS2716::len()].read(address as usize % rom::TMS2716::len()),
      0x2000..=0x3FFF => self.ram[address as usize / ram::I2107B::len()].read(address as usize % ram::I2107B::len()),
      _ => panic!("Invalid read at address {}", address),
    }
  }
  fn write_mem<T: chips::WriteArr>(&mut self, address: u16, value: T) {
    match address {
      0..=0x1FFF => warn!("Attempted to write to rom address {}", address),
      0x2000..=0x3FFF => self.ram[address as usize / ram::I2107B::len()].write(address as usize % ram::I2107B::len(), value),
      _ => panic!("Invalid read at address {}", address),
    }
  }
}