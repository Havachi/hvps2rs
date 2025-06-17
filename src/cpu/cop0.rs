use crate::interrupt::InterruptState;

pub struct Cop0 {
	sr: u32,
	cause: u32,
	epc: u32,
}

impl Cop0 {
	pub fn new() -> Self {
		Self {
			sr: 0,
			cause: 0,
			epc: 0,
		}
	}

	pub fn sr(&self) -> u32 {
		self.sr
	}

	pub fn set_sr(&mut self, sr: u32) {
		self.sr = sr;
	}

	pub fn cause(&self, irq_state: InterruptState) -> u32 {
		self.cause | ((irq_state.active() as u32) << 10)
	}

	pub fn set_cause(&mut self, v: u32) {
		self.cause &= !0x300;
		self.cause |= v & 0x300;
	}

	pub fn epc(&self) -> u32 {
		self.epc
	}

	pub fn cache_isolated(&self) -> bool {
		self.sr & 0x10000 != 0
	}

	pub fn enter_exeception(&mut self, cause: Exception, pc: u32, in_delay_slot: bool) -> u32 {
		let mode = self.sr & 0x3F;
		self.sr &= !0x3F;
		self.sr |= (mode << 2) & 0x3F;
		self.cause &= !0x7C;
		self.cause |= (cause as u32) << 2;
		if in_delay_slot {
			self.epc = pc.wrapping_sub(4);
			self.cause |= 1 << 31;
		} else {
			self.epc = pc;
			self.cause &= !(1 << 31);
		}

		match self.sr & (1 << 22) != 0 {
			true => 0xBFC0_0180,
			false => 0x8000_0080,
		}
	}

	pub fn return_from_exception(&mut self) {
		let mode = self.sr & 0x3F;
		self.sr &= !0xF;
		self.sr |= mode >> 2;
	}

	fn irq_enable(&self) -> bool {
		self.sr & 1 != 0
	}

	pub fn irq_active(&self, irq_state: InterruptState) -> bool {
		let cause = self.cause(irq_state);
		let pending = (cause & self.sr) & 0x700;
		self.irq_enable() && pending != 0
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Exception {
	Interrupt = 0x0,
	LoadAddressError = 0x4,
	StoreAddressError = 0x5,
	SysCall = 0x8,
	Break = 0x9,
	IllegalInstruction = 0xA,
	CoprocessorError = 0xB,
	Overflow = 0xC,
}