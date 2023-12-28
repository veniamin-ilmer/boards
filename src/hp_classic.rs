//! HP Classic Calculators includes HP-35 and HP-45.
//! Each clock cycle ends up taking 280 microseconds. (3.671 kHz)

const ROM_CHIP_LEN: usize = 320;  /// 256 * 10 bits = 2560 bits of ROM data. 2560 / 8 = 320 bytes
use log::trace;
use chips::{rom,cpu,ram};
use arbitrary_int::{
  u3,   //ROM #
  u6,   //Key code
  u10,  //ROM opcode
  u14,  //Word Select
};

pub struct Board {
  pub anr: cpu::HP_AnR,
  pub cnt: cpu::HP_CnT,
  pub roms: Vec<rom::HP_ROM>,
  pub ram: ram::HP_RAM,
}

impl Board {
  pub fn new(packed_rom_data: Vec<u8>) -> Self {
    let mut roms = vec![];
    let mut rom_num = u3::new(0);
    for chunk in packed_rom_data.chunks(ROM_CHIP_LEN) {
      let mut padded_chunk = Vec::from(chunk);
      padded_chunk.resize_with(ROM_CHIP_LEN, Default::default); //This is needed for the last chunk if it is less than the total.
      roms.push(rom::HP_ROM::new(padded_chunk.try_into().unwrap(), rom_num));
      rom_num += u3::new(1);
    }
    Self {
      anr: cpu::HP_AnR::new(),
      cnt: cpu::HP_CnT::new(),
      roms,
      ram: ram::HP_RAM::new(),
    }
  }

  pub fn run_cycle(&mut self, keyboard_code: Option<u6>) {
    let mut opcode = u10::new(0);
    let mut word_select_data = u14::new(0);
    for rom in &mut self.roms {
      let (opcode_rom, word_select_data_rom) = rom.read(self.cnt.next_address);
      opcode |= opcode_rom;
      word_select_data |= word_select_data_rom;
    }
    
    //ROM Select decoder. (Should this be in a separate chip??)
    if opcode.value() & 0b1111111 == 0b0010000 {
      let rom_num = (opcode.value() >> 7) as u8;
      trace!("SELECT ROM {}", rom_num);
      for rom in &mut self.roms {
        rom.select_rom(u3::new(rom_num));
      }
    }
    
    word_select_data |= self.cnt.run_cycle(opcode, self.anr.next_carry, keyboard_code);
    let ram_data = self.ram.run_cycle(opcode, self.anr.c);
    self.anr.run_cycle(opcode, word_select_data, ram_data);
    self.cnt.print();
    self.anr.print();
  }

}
