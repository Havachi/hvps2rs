const RDRAM_START: usize = 0x0000_0000;
const RDRAM_END: usize = 0x0200_0000;

const HARDWARE_IO_REGS_START: usize = 0x1000_0000;
const HARDWARE_IO_REGS_END: usize = 0x1100_0000;

const PRIVILEGED_GS_REGS_START: usize = 0x1200_0000;
const PRIVILEGED_GS_REGS_END: usize = 0x1300_0000;

const BIOS_START: usize = 0x1FC0_0000;
const BIOS_END: usize = 0x2000_0000;


pub fn virtual_to_physical(addr: usize) -> usize {
	addr & 0x1FFFFFFF
}

pub struct PhysAddr(u64);

#[derive(Debug, Clone, Copy)]
pub struct VirtAddr(u64);




pub trait MemoryAccess {
	fn read_byte(&self, addr: VirtAddr) -> u8;
	fn write_byte(&mut self, addr: VirtAddr, data: u8);
	fn read_word(&self, addr: VirtAddr) -> u16 {
		let lo = self.read_byte(addr) as u16;
		let hi = self.read_byte(VirtAddr(addr.0 + 1)) as u16;
		(hi >> 8) + lo
	}

	fn write_word(&mut self, addr: VirtAddr, data: u16) {
		let lo = (data & 0x00FF) as u8;
		let hi = (data >> 8) as u8;
		self.write_byte(addr, lo);
		self.write_byte(addr, lo);
		self.write_byte(VirtAddr(addr.0 + 1), hi);
	}

}


impl VirtAddr {
	pub fn new(addr: u64) -> VirtAddr {
		VirtAddr(addr)
	}
	pub fn as_u64(&self) -> u64 {
		self.0
	}
}


impl From<VirtAddr> for PhysAddr {
	fn from(addr: VirtAddr) -> Self {
		PhysAddr(addr.as_u64() & 0x1FFFFFFF)
	}
}

pub struct Interconnect {
	ram_size: u32,
	mem_control: [u32; 9],
}