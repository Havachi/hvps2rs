use crate::bios::Bios;

const RDRAM_START: usize = 0x0000_0000;
const RDRAM_END: usize = 0x0200_0000;

const HARDWARE_IO_REGS_START: usize = 0x1000_0000;
const HARDWARE_IO_REGS_END: usize = 0x1100_0000;

const PRIVILEGED_GS_REGS_START: usize = 0x1200_0000;
const PRIVILEGED_GS_REGS_END: usize = 0x1300_0000;

const BIOS_START: usize = 0x1FC0_0000;
const BIOS_END: usize = 0x2000_0000;


pub struct Interconnect {
	bios: Bios,

	ram_size: u32,
	mem_control: [u32; 9],
}

impl Interconnect {

}

pub trait Addressable {
	fn size() -> u8;
}

pub struct Byte;

impl Addressable for Byte {
	fn size() -> u8 {
		1
	}
}

pub struct HalfWord;

impl Addressable for HalfWord {
    fn size() -> u8 {
        2
    }
}

pub struct Word;

impl Addressable for Word {
    fn size() -> u8 {
        4
    }
}