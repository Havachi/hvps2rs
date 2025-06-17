///Direct Memory Access Controller
pub mod dmac;
pub mod gif;
pub mod ipu;
pub mod timer;
pub mod vif;
pub mod vu;

pub mod registers;
pub mod fpu;

struct R5900cpu {

}

impl R5900cpu {
	pub fn reserve(){}
	pub fn shutdown(){}
	pub fn reset(){}
	pub fn step(){}
	pub fn execute(){}
	pub fn exit_execution(){}
	pub fn cancel_instruction(){}
	pub fn clear(addr: u32, size: u32){}
}

macro_rules! EXC_CODE {
	() => {
		
	};
}