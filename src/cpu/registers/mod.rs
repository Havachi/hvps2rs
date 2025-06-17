use modular_bitfield::{bitfield, specifiers::*};


pub union GPreg {
	UQ: u128,
	SQ: u128,
	UD: [u64; 2],
	SD: [i64; 2],
	UL: [u32; 4],
	SL: [i32; 4],
	US: [u16; 8],
	SS: [i16; 8],
	UC: [u8; 16],
	SC: [i8; 16],
}

pub union GPreg64 {
	UD: [u64; 1],
	SD: [i64; 1],
	UL: [u32; 2],
	SL: [i32; 2],
	US: [u16; 4],
	SS: [i16; 4],
	UC: [u8; 8],
	SC: [i8; 8],
}

pub enum GPRegs{
	N{
		r0: GPreg,
		at: GPreg,
		v0: GPreg,
		v1: GPreg,
		a0: GPreg,
		a1: GPreg,
		a2: GPreg,
		a3: GPreg,
		t0: GPreg,
		t1: GPreg,
		t2: GPreg,
		t3: GPreg,
		t4: GPreg,
		t5: GPreg,
		t6: GPreg,
		t7: GPreg,
		s0: GPreg,
		s1: GPreg,
		s2: GPreg,
		s3: GPreg,
		s4: GPreg,
		s5: GPreg,
		s6: GPreg,
		s7: GPreg,
		t8: GPreg,
		t9: GPreg,
		k0: GPreg,
		k1: GPreg,
		gp: GPreg,
		sp: GPreg,
		s8: GPreg,
		ra: GPreg,
	},
	R([GPreg; 32]),
}

enum PCCR {
	B(PCCRBitfield),
	VAL(u32),
}

#[bitfield(bits = 32)]
#[repr(C, packed)]
pub struct PCCRBitfield {
	pad0: B1,
	exl0: B1,
	k0: B1,
	s0: B1,
	u0: B1,
	event0: B5,
	pad1: B1,
	exl1: B1,
	k1: B1,
	s1: B1,
	u1: B1,
	event1: B5,
	reserved: B11,
	cte: B1
} 

pub enum PERFregs {
	N { 
		pccr: PCCR,
		pcr0: u32,
		pcr1: u32,
		pad: u32,
	},
	R ([u32; 4]),
}

#[bitfield(bits = 32)]

#[repr(C, packed)]
pub struct CP0Status{
	ie: B1,
	exl: B1,
	erl: B1,
	ksu: B2,
	#[skip] unused0: B3,
	im: B8,
	eie: B1,
	edi:B1,
	ch: B1,
	#[skip] unused1: B3,
	bev: B1,
	dev: B1,
	#[skip] unused2: B2,
	fr: B1,
	#[skip] unused3: B1,
	cu: B4,
}

pub struct CP0regs {
	index: u32,
	random: u32,
	entry_lo0: u32,
	entry_lo1: u32,
	context: u32,
	pagemask: u32,
	wired: u32,
	reserverd0:u32,
	badvaddr: u32,
	count: u32,
	entry_hi:u32,
	compare:u32,
	status: CP0Status,
	cause: u32,
	epc: u32,
	prid: u32,
	config: u32,
	lladdr: u32,
	watch_lo: u32,
	watch_hi: u32,
	xcontext: u32,
	reserved1: u32,
	reserved2: u32,
	debug: u32,
	depc: u32,
	perfcnt: u32,
	errctl: u32,
	cachederr: u32,
	tag_lo: u32,
	tag_hi: u32,
	error_epc: u32,
	desave:u32,
}

struct CPURegister {
	gpr: GPRegs,
	hi: GPreg,
	lo: GPreg,
	cp0: CP0regs,
	sa: u32,
	is_delay_slot: u32,
	///Program counter
	pc: u32,
	///Current Intruction
	code: u32,
	pref: PERFregs,
	e_cycles: [u32; 32],
	s_cycles: [u32; 32],
	cycles: u32,
	interrupt: u32,
	branch: i32,
	opmode: i32,
	tempcycles: u32,
	dmastall: u32,
	pc_writeback: u32,
	next_event_cycles: u32,
	last_event_cycles: u32,
	last_cop0_cycles: u32,
	last_perf_cycles: [u32; 2],
}

impl CP0regs {
	pub fn get_status_val(&self) -> u32 {
		let data: u32 = 
			self.status.ie() as u32 +
			self.status.exl() as u32 +
			self.status.erl() as u32 +
			self.status.ksu() as u32 +
			self.status.im() as u32 +
			self.status.eie() as u32 +
			self.status.edi() as u32 +
			self.status.ch() as u32 +
			self.status.bev() as u32 +
			self.status.dev() as u32 +
			self.status.fr() as u32 +
			self.status.cu() as u32;
		data
	}
}

union FPRreg {
	f: f32,
	ul: u32,
	sl: i32,
}

struct FPURegisters {
	fpr: [FPRreg; 32],
	fprc: [u32; 32],
	acc: FPRreg,
	acc_flags: u32,
}

