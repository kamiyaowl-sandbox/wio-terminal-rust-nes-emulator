use super::interface::{SystemBus, EmulateControl};

pub const SIZE: usize = 0x8000;

pub struct ProgramRom {
    pub rom: [u8; SIZE],
}

impl SystemBus for ProgramRom {
    fn read_u8(&self, addr: u16) -> u8 {
        assert!((addr as usize) < self.rom.len());
        self.rom[addr as usize]
    }
    fn write_u8(&mut self, addr: u16, data: u8) {
        panic!("Memory Write Request Error. PRG-ROM. addr:{:x}, data:{:x}", addr, data);
    }
}

impl EmulateControl for ProgramRom {
    fn reset(&mut self){
        self.rom = [0; SIZE];
    }
    fn store(&self, read_callback: fn(usize, u8)){
        for i in 0..self.rom.len() {
            read_callback(i, self.rom[i]);
        }
    }
    fn restore(&mut self, write_callback: fn(usize) -> u8){
        for i in 0..self.rom.len() {
            let data = write_callback(i);
             self.rom[i] = data;
        }
    }
}