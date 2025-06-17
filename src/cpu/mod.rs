mod cop0;
mod gte;
use std::fmt::Display;
use crate::{cpu::cop0::Cop0, cpu::gte::Gte, memory::Interconnect};

pub struct Cpu {
	///Program counter
	pc: u32,
	next_pc: u32,
	current_pc: u32,
	///General Purpose Registers
	regs: [u32; 32],
	///High register
	hi: u32,
	///Low register
	lo: u32,
	///Instruction Cache (256 4-word cachelines)
	icache: InstrCacheLines,
	///Memory Interface
	inter: Interconnect,
	/// Coprocessor 0: System control
	cop0: Cop0,
	/// Coprocessor 2: Geometry Transform Engine
	gte: Gte,
	load: (RegisterIndex, u32),
	branch: bool,
	delay_slot: bool,
	debug_on_break: bool,
}

#[derive(Clone, Copy)]
struct Instruction(u32);

impl Instruction {
	/// Return bits [31:26] of the instruction
	fn function(self) -> u32 {
		let Instruction(op) = self;
		op >> 26
	}
	/// Return bits [5:0] of the instruction
	fn subfunction(self) -> u32 {
		let Instruction(op) = self;
		op & 0x3f
	}
	/// Return coprocessor opcode in bits [25:21]
	fn cop_opcode(self) -> u32 {
		let Instruction(op) = self;
		(op >> 21) & 0x1f
	}
	/// Return register index in bits [25:21]
	fn s(self) -> RegisterIndex {
		let Instruction(op) = self;
		RegisterIndex((op >> 21) & 0x1f)
	}
	/// Return register index in bits [20:16]
	fn t(self) -> RegisterIndex {
		let Instruction(op) = self;
		RegisterIndex((op >> 16) & 0x1f)
	}
	/// Return register index in bits [15:11]
	fn d(self) -> RegisterIndex {
		let Instruction(op) = self;

		RegisterIndex((op >> 11) & 0x1f)
	}
	/// Return immediate value in bits [16:0]
	fn imm(self) -> u32 {
		let Instruction(op) = self;
		op & 0xFFFF
	}
	/// Return immediate value in bits [16:0] as a sign-extended 32bit value
	fn imm_se(self) -> u32 {
		let Instruction(op) = self;
		let v = (op & 0xffff) as i16;
		v as u32
	}
	/// Shift Immediate values are stored in bits [10:6]
	fn shift(self) -> u32 {
		let Instruction(op) = self;
		(op >> 6) & 0x1f
	}

	/// Jump target stored in bits [25:0]
	fn imm_jump(self) -> u32 {
		let Instruction(op) = self;
		op & 0x3ffffff
	}
    /// Return true if the instruction contains a GTE/COP2 opcode
    fn is_gte_op(self) -> bool {
        self.function() == 0b010001
    }
}

impl Display for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:08X}", self.0)
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct RegisterIndex(u32);

#[derive(Clone, Copy)]
struct InstrCacheLines {
	tag_valid: u32,
	line: [Instruction; 4]
}

impl InstrCacheLines {
	fn new() -> Self {
		Self {
			tag_valid: 0x0,
			line: [Instruction(0); 4]
		}
	}
	/// Return the cacheline's tag
	fn tag(&self) -> u32 {
		self.tag_valid & 0xFFFF_F000
	}
	/// Return the cacheline's first valid word
	fn valid_index(&self) -> u32 {
		(self.tag_valid >> 2) & 0x7
	}
    /// Set the cacheline's tag and valid bits. `pc` is the first
    /// valid PC in the cacheline.
	fn set_tag_valid(&mut self, pc: u32) {
		self.tag_valid = pc & 0x7FFF_F00C;
	}
    /// Invalidate the entire cacheline by pushing the index out of
    /// range. Doesn't change the tag or contents of the line.
	fn invalidate(&mut self) {
		self.tag_valid |= 0x10;
	}

	fn instruction(&self, index: u32) -> Instruction {
		self.line[index as usize]
	}

	fn set_instruction(&mut self, index: u32, instruction: Instruction) {
		self.line[index as usize] = instruction
	}
}

impl Default for InstrCacheLines {
	fn default() -> Self {
		InstrCacheLines::new()
	}
}
