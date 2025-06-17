#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {
	VBlank = 0,
	CdRom = 2,
	Dma = 3,
	Timer0 = 4,
	Timer1 = 5,
	Timer2 = 6,
	PadMemCard = 7,
}

pub struct InterruptState {
	status: u16,
	mask: u16,
}

impl InterruptState {
	pub fn new() -> InterruptState {
		InterruptState {
			status: 0,
			mask: 0,
		}
	}

	pub fn active(self) -> bool {
		(self.status & self.mask) != 0
	}

	pub fn status(self) -> u16 {
		self.status
	}
	pub fn ack(&mut self, ack: u16) {
		self.status &= ack;
	}
	pub fn mask(self) -> u16 {
		self.mask
	}

	pub fn set_mask(&mut self, mask: u16) {
		let supported = [
			Interrupt::VBlank,
			Interrupt::CdRom,
			Interrupt::Dma,
			Interrupt::Timer0,
			Interrupt::Timer1,
			Interrupt::Timer2,
			Interrupt::PadMemCard,
		];

		let rem = supported.iter().fold(mask,
			|mask, &it| mask & !(1 << it as u16)
		);

		if rem != 0 {
			panic!("Unsupported interrupt: {:04X}", rem);
		}
		self.mask = mask;
	}

	pub fn assert(&mut self, which: Interrupt) {
		self.status |= 1 << (which as usize);
	}
}

