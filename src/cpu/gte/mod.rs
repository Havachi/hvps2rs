mod divider;

#[derive(Debug)]
pub struct Gte {
	ofx: i32,
	ofy: i32,
	h: u16,
	dqa: i16,
	dqb: i32,
	zsf3: i16,
	zsf4: i16,
	matrices: [[[i16; 3]; 3]; 3],
	control_vectors: [[i32; 3]; 4],
	flags: u32,
	v: [[i16; 3]; 4],
	mac: [i32; 4],
	otz: u16,
	rgb: (u8, u8, u8, u8),
	ir: [i16; 4],
	xy_fifo: [(i16, i16); 4],
	z_fifo: [u16; 4],
	rgb_fifo: [(u8,u8,u8,u8); 3],
	lzcs: u32,
	lzcr: u8,
	reg_23: u32,
}

impl Gte {
	pub fn new() -> Gte {
		Gte {
			ofx: 0,
			ofy: 0,
			h: 0,
			dqa: 0,
			dqb: 0,
			zsf3:0,
			zsf4: 0,
			matrices: [[[0; 3]; 3]; 3],
			control_vectors: [[0; 3]; 4],
			flags: 0,
			v: [[0; 3]; 4],
			mac: [0; 4],
			rgb: (0, 0, 0, 0),
			otz: 0,
			ir: [0; 4],
			xy_fifo: [(0, 0); 4],
			z_fifo: [0; 4],
			rgb_fifo: [(0, 0, 0, 0); 3],
			lzcs: 0,
			lzcr: 32,
			reg_23: 0,
		}
	}

	pub fn command(&mut self, command: u32) {
		let opcode = command & 0x3F;
		let config = CommandConfig::from_command(command);
		self.flags = 0;
		match opcode {
			0x01 => self.cmd_rtps(config),
			0x06 => self.cmd_nclip(),
			0x0c => self.cmd_op(config),
			0x10 => self.cmd_dpcs(config),
			0x11 => self.cmd_intpl(config),
			0x12 => self.cmd_mvmva(config),
			0x13 => self.cmd_ncds(config),
			0x16 => self.cmd_ncdt(config),
			0x1b => self.cmd_nccs(config),
			0x1c => self.cmd_cc(config),
			0x1e => self.cmd_ncs(config),
			0x20 => self.cmd_nct(config),
			0x28 => self.cmd_sqr(config),
			0x29 => self.cmd_dcpl(config),
			0x2a => self.cmd_dpct(config),
			0x2d => self.cmd_avsz3(),
			0x2e => self.cmd_avsz4(),
			0x30 => self.cmd_rtpt(config),
			0x3d => self.cmd_gpf(config),
			0x3e => self.cmd_gpl(config),
			0x3f => self.cmd_ncct(config),
			_ => panic!("Unhandled GTE opcode {:02x}", opcode),
		}
		let msb = self.flags & 0x7F87_E000 != 0;
		self.flags |= (msb as u32) << 31;
	}

	pub fn control(&self, reg: u32) -> u32 {
		match reg {
			0 => {
				let matrix = &self.matrices[Matrix::Rotation.index()];

				let v0 = matrix[0][0] as u16 as u32;
				let v1 = matrix[0][1] as u16 as u32;

				v0 | v1 << 16
			}
			1 => {
				let matrix = &self.matrices[Matrix::Rotation.index()];

				let v0 = matrix[0][2] as u16 as u32;
				let v1 = matrix[1][0] as u16 as u32;

				v0 | v1 << 16
			}
			2 => {
				let matrix = &self.matrices[Matrix::Rotation.index()];

				let v0 = matrix[1][1] as u16 as u32;
				let v1 = matrix[1][2] as u16 as u32;

				v0 | v1 << 16
			}
			3 => {
				let matrix = &self.matrices[Matrix::Rotation.index()];

				let v0 = matrix[2][0] as u16 as u32;
				let v1 = matrix[2][1] as u16 as u32;

				v0 | v1 << 16
			}
			4 => {
				let matrix = &self.matrices[Matrix::Rotation.index()];

				matrix[2][2] as u32
			}
			5..=7 => {
				let index = ControlVector::Translation.index();
				let vector = &self.control_vectors[index];

				vector[reg as usize - 5] as u32
			}
			8 => {
				let matrix = &self.matrices[Matrix::Light.index()];

				let v0 = matrix[0][0] as u16 as u32;
				let v1 = matrix[0][1] as u16 as u32;

				v0 | v1 << 16
			}
			9 => {
				let matrix = &self.matrices[Matrix::Light.index()];

				let v0 = matrix[0][2] as u16 as u32;
				let v1 = matrix[1][0] as u16 as u32;

				v0 | v1 << 16
			}
			10 => {
				let matrix = &self.matrices[Matrix::Light.index()];

				let v0 = matrix[1][1] as u16 as u32;
				let v1 = matrix[1][2] as u16 as u32;

				v0 | v1 << 16
			}
			11 => {
				let matrix = &self.matrices[Matrix::Light.index()];

				let v0 = matrix[2][0] as u16 as u32;
				let v1 = matrix[2][1] as u16 as u32;

				v0 | v1 << 16
			}
			12 => {
				let matrix = &self.matrices[Matrix::Light.index()];

				matrix[2][2] as u32
			}
			13..=15 => {
				let index = ControlVector::BackgroundColor.index();
				let vector = &self.control_vectors[index];

				vector[reg as usize - 13] as u32
			}
			16 => {
				let matrix = &self.matrices[Matrix::Color.index()];

				let v0 = matrix[0][0] as u16 as u32;
				let v1 = matrix[0][1] as u16 as u32;

				v0 | v1 << 16
			}
			17 => {
				let matrix = &self.matrices[Matrix::Color.index()];

				let v0 = matrix[0][2] as u16 as u32;
				let v1 = matrix[1][0] as u16 as u32;

				v0 | v1 << 16
			}
			18 => {
				let matrix = &self.matrices[Matrix::Color.index()];

				let v0 = matrix[1][1] as u16 as u32;
				let v1 = matrix[1][2] as u16 as u32;

				v0 | v1 << 16
			}
			19 => {
				let matrix = &self.matrices[Matrix::Color.index()];

				let v0 = matrix[2][0] as u16 as u32;
				let v1 = matrix[2][1] as u16 as u32;

				v0 | v1 << 16
			}
			20 => {
				let matrix = &self.matrices[Matrix::Color.index()];

				matrix[2][2] as u32
			}
			21..=23 => {
				let index = ControlVector::FarColor.index();
				let vector = &self.control_vectors[index];

				vector[reg as usize - 21] as u32
			}
			24 => self.ofx as u32,
			25 => self.ofy as u32,
			26 => self.h as i16 as u32,
			27 => self.dqa as u32,
			28 => self.dqb as u32,
			29 => self.zsf3 as u32,
			30 => self.zsf4 as u32,
			31 => self.flags,
			_ => panic!("Unhandled GTE control register {}", reg),
		}
	}

	pub fn set_control(&mut self, reg: u32, val: u32) {
		match reg {
			0 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Rotation.index()];

				matrix[0][0] = v0;
				matrix[0][1] = v1;
			}
			1 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Rotation.index()];

				matrix[0][2] = v0;
				matrix[1][0] = v1;
			}
			2 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Rotation.index()];

				matrix[1][1] = v0;
				matrix[1][2] = v1;
			}
			3 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Rotation.index()];

				matrix[2][0] = v0;
				matrix[2][1] = v1;
			}
			4 => {
				let matrix = &mut self.matrices[Matrix::Rotation.index()];

				matrix[2][2] = val as i16;
			}
			5..=7 => {
				let index = ControlVector::Translation.index();
				let vector = &mut self.control_vectors[index];

				vector[reg as usize - 5] = val as i32;
			}
			8 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Light.index()];

				matrix[0][0] = v0;
				matrix[0][1] = v1;
			}
			9 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Light.index()];

				matrix[0][2] = v0;
				matrix[1][0] = v1;
			}
			10 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Light.index()];

				matrix[1][1] = v0;
				matrix[1][2] = v1;
			}
			11 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Light.index()];

				matrix[2][0] = v0;
				matrix[2][1] = v1;
			}
			12 => {
				let matrix = &mut self.matrices[Matrix::Light.index()];

				matrix[2][2] = val as i16;
			}
			13..=15 => {
				let index = ControlVector::BackgroundColor.index();
				let vector = &mut self.control_vectors[index];

				vector[reg as usize - 13] = val as i32;
			}
			16 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Color.index()];

				matrix[0][0] = v0;
				matrix[0][1] = v1;
			}
			17 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Color.index()];

				matrix[0][2] = v0;
				matrix[1][0] = v1;
			}
			18 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Color.index()];

				matrix[1][1] = v0;
				matrix[1][2] = v1;
			}
			19 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				let matrix = &mut self.matrices[Matrix::Color.index()];

				matrix[2][0] = v0;
				matrix[2][1] = v1;
			}
			20 => {
				let matrix = &mut self.matrices[Matrix::Color.index()];

				matrix[2][2] = val as i16;
			}
			21..=23 => {
				let index = ControlVector::FarColor.index();
				let vector = &mut self.control_vectors[index];

				vector[reg as usize - 21] = val as i32;
			}
			24 => self.ofx = val as i32,
			25 => self.ofy = val as i32,
			26 => self.h = val as u16,
			27 => self.dqa = val as i16,
			28 => self.dqb = val as i32,
			29 => self.zsf3 = val as i16,
			30 => self.zsf4 = val as i16,
			31 => {
				self.flags = val & 0x7ffff00;

				let msb = val & 0x7f87e000 != 0;

				self.flags |= (msb as u32) << 31;
			}
			_  => panic!("Unhandled GTE control register {} {:x}", reg, val),
		}
	}

	pub fn data(&self, reg: u32) -> u32 {
		let rgbx_to_u32 = | rgbx | -> u32 {
			let (r, g, b, x) = rgbx;

			let r = r as u32;
			let g = g as u32;
			let b = b as u32;
			let x = x as u32;

			r | (g << 8) | (b << 16) | (x << 24)
		};

		let xy_to_u32 = | xy | -> u32 {
			let (x, y) = xy;

			let x = x as u16;
			let y = y as u16;

			(x as u32) | ((y as u32) << 16)
		};

		match reg {
			0 => {
				let v0 = self.v[0][0] as u16 as u32;
				let v1 = self.v[0][1] as u16 as u32;

				v0 | v1 << 16
			},
			1 => self.v[0][2] as u32,
			2 => {
				let v0 = self.v[1][0] as u16 as u32;
				let v1 = self.v[1][1] as u16 as u32;

				v0 | v1 << 16
			},
			3 => self.v[1][2] as u32,
			4 => {
				let v0 = self.v[2][0] as u16 as u32;
				let v1 = self.v[2][1] as u16 as u32;

				v0 | v1 << 16
			},
			5 => self.v[2][2] as u32,
			6 => rgbx_to_u32(self.rgb),
			7 => self.otz as u32,
			8 => self.ir[0] as u32,
			9 => self.ir[1] as u32,
			10 => self.ir[2] as u32,
			11 => self.ir[3] as u32,
			12 => xy_to_u32(self.xy_fifo[0]),
			13 => xy_to_u32(self.xy_fifo[1]),
			14 => xy_to_u32(self.xy_fifo[2]),
			15 => xy_to_u32(self.xy_fifo[3]),
			16 => self.z_fifo[0] as u32,
			17 => self.z_fifo[1] as u32,
			18 => self.z_fifo[2] as u32,
			19 => self.z_fifo[3] as u32,
			20 => rgbx_to_u32(self.rgb_fifo[0]),
			21 => rgbx_to_u32(self.rgb_fifo[1]),
			22 => rgbx_to_u32(self.rgb_fifo[2]),
			23 => self.reg_23,
			24 => self.mac[0] as u32,
			25 => self.mac[1] as u32,
			26 => self.mac[2] as u32,
			27 => self.mac[3] as u32,
			28 | 29 => {
				let saturate = | v | {
					if v < 0 {
						0
					} else if v > 0x1f {
						0x1f
					} else {
						v as u32
					}
				};

				let a = saturate(self.ir[1] >> 7);
				let b = saturate(self.ir[2] >> 7);
				let c = saturate(self.ir[3] >> 7);

				a | (b << 5) | (c << 10)
			}
			30 => self.lzcs,
			31 => self.lzcr as u32,
			_  => panic!("Unhandled GTE data register {}", reg),
		}
	}

	pub fn set_data(&mut self, reg: u32, val: u32) {

		let val_to_rgbx = || -> (u8, u8, u8, u8) {
			let r = val as u8;
			let g = (val >> 8) as u8;
			let b = (val >> 16) as u8;
			let x = (val >> 24) as u8;

			(r, g, b, x)
		};

		let val_to_xy = || -> (i16, i16) {
			let x = val as i16;
			let y = (val >> 16) as i16;

			(x, y)
		};

		match reg {
			0 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				self.v[0][0] = v0;
				self.v[0][1] = v1;
			}
			1 => self.v[0][2] = val as i16,
			2 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				self.v[1][0] = v0;
				self.v[1][1] = v1;
			}
			3 => self.v[1][2] = val as i16,
			4 => {
				let v0 = val as i16;
				let v1 = (val >> 16) as i16;

				self.v[2][0] = v0;
				self.v[2][1] = v1;
			}
			5 => self.v[2][2] = val as i16,
			6 => self.rgb = val_to_rgbx(),
			7 => self.otz = val as u16,
			8 => self.ir[0] = val as i16,
			9 => self.ir[1] = val as i16,
			10 => self.ir[2] = val as i16,
			11 => self.ir[3] = val as i16,
			12 => self.xy_fifo[0] = val_to_xy(),
			13 => self.xy_fifo[1] = val_to_xy(),
			14 => {
				let xy = val_to_xy();
				self.xy_fifo[2] = xy;
				self.xy_fifo[3] = xy;
			}
			15 => {
				self.xy_fifo[3] = val_to_xy();
				self.xy_fifo[0] = self.xy_fifo[1];
				self.xy_fifo[1] = self.xy_fifo[2];
				self.xy_fifo[2] = self.xy_fifo[3];
			}
			16 => self.z_fifo[0] = val as u16,
			17 => self.z_fifo[1] = val as u16,
			18 => self.z_fifo[2] = val as u16,
			19 => self.z_fifo[3] = val as u16,
			20 => self.rgb_fifo[0] = val_to_rgbx(),
			21 => self.rgb_fifo[1] = val_to_rgbx(),
			22 => self.rgb_fifo[2] = val_to_rgbx(),
			23 => self.reg_23 = val,
			24 => self.mac[0] = val as i32,
			25 => self.mac[1] = val as i32,
			26 => self.mac[2] = val as i32,
			27 => self.mac[3] = val as i32,
			28 => {
				let to_ir = |v: u32| -> i16 {
					((v & 0x1f) << 7) as i16
				};

				self.ir[0] = to_ir(val);
				self.ir[1] = to_ir(val >> 5);
				self.ir[2] = to_ir(val >> 10);
			}
			29 => (),
			30 => {
				self.lzcs = val;
				let tmp =
					if (val >> 31) & 1 != 0 {
						!val
					} else {
						val
					};

				self.lzcr = tmp.leading_zeros() as u8;
			}
			31 => warn!("Write to read-only GTE data register 31"),
			_  => unreachable!(),
		}
	}

	fn cmd_rtps(&mut self, config: CommandConfig) {
		let projection_factor = self.do_rtp(config, 0);
		self.depth_queuing(projection_factor);
	}

	fn cmd_nclip(&mut self) {
		let (x0, y0) = self.xy_fifo[0];
		let (x1, y1) = self.xy_fifo[1];
		let (x2, y2) = self.xy_fifo[2];

		let (x0, y0) = (x0 as i32, y0 as i32);
		let (x1, y1) = (x1 as i32, y1 as i32);
		let (x2, y2) = (x2 as i32, y2 as i32);

		let a = x0 * (y1 - y2);
		let b = x1 * (y2 - y0);
		let c = x2 * (y0 - y1);

		let sum = a as i64 + b as i64 + c as i64;

		self.check_mac_overflow(sum);

		self.mac[0] = sum as i32;
	}

	fn cmd_op(&mut self, config: CommandConfig) {
		let rm = Matrix::Rotation.index();

		let ir1 = self.ir[1] as i32;
		let ir2 = self.ir[2] as i32;
		let ir3 = self.ir[3] as i32;

		let r0 = self.matrices[rm][0][0] as i32;
		let r1 = self.matrices[rm][1][1] as i32;
		let r2 = self.matrices[rm][2][2] as i32;

		let shift = config.shift;

		self.mac[1] = (r1 * ir3 - r2 * ir2) >> shift;
		self.mac[2] = (r2 * ir1 - r0 * ir3) >> shift;
		self.mac[3] = (r0 * ir2 - r1 * ir1) >> shift;

		self.mac_to_ir(config);
	}

	fn cmd_dpcs(&mut self, config: CommandConfig) {
		let fc = ControlVector::FarColor.index();

		let (r, g, b, _) = self.rgb;

		let col = [r, g, b];
		for i in 0..3 {
			let fc = (self.control_vectors[fc][i] as i64) << 12;
			let col = (col[i] as i64) << (4 + 12);

			let sub = fc - col;

			let tmp = (self.i64_to_i44(i as u8, sub) >> config.shift) as i32;

			let ir0 = self.ir[0] as i64;

			let sat = self.i32_to_i16_saturate(CommandConfig::from_command(0),
											   i as u8, tmp) as i64;

			let res = self.i64_to_i44(i as u8, col + ir0 * sat);

			self.mac[i + 1] = (res >> config.shift) as i32;
		}

		self.mac_to_ir(config);
		self.mac_to_rgb_fifo();
	}

	fn cmd_dcpl(&mut self, config: CommandConfig) {
		let fc = ControlVector::FarColor.index();

		let (r, g, b, _) = self.rgb;

		let col = [r, g, b];
		for i in 0..3 {
			let fc = (self.control_vectors[fc][i] as i64) << 12;
			let ir = self.ir[i + 1] as i32;
			let col = (col[i] as i32) << 4;

			let shading = (col * ir) as i64;

			let tmp = fc - shading;

			let tmp = (self.i64_to_i44(i as u8, tmp) >> config.shift) as i32;

			let ir0 = self.ir[0] as i64;

			let res = self.i32_to_i16_saturate(CommandConfig::from_command(0),
											   i as u8, tmp) as i64;

			let res = self.i64_to_i44(i as u8, shading + ir0 * res);

			self.mac[i + 1] = (res >> config.shift) as i32;
		}

		self.mac_to_ir(config);
		self.mac_to_rgb_fifo();
	}

	fn cmd_dpct(&mut self, config: CommandConfig) {
		// Each call uses the oldest entry in the RGB FIFO and pushes
		// the result at the top so the three calls will process and
		// replace the entire contents of the FIFO
		self.do_dpc(config);
		self.do_dpc(config);
		self.do_dpc(config);
	}
	fn do_dpc(&mut self, config: CommandConfig) {
		let fc = ControlVector::FarColor.index();

		let (r, g, b, _) = self.rgb_fifo[0];

		let col = [r, g, b];

		for i in 0..3 {
			let fc = (self.control_vectors[fc][i] as i64) << 12;
			let col = (col[i] as i64) << (4 + 12);

			let sub = fc - col;

			let tmp = (self.i64_to_i44(i as u8, sub) >> config.shift) as i32;

			let ir0 = self.ir[0] as i64;

			let sat = self.i32_to_i16_saturate(CommandConfig::from_command(0),
											   i as u8, tmp) as i64;

			let res = self.i64_to_i44(i as u8, col + ir0 * sat);

			self.mac[i + 1] = (res >> config.shift) as i32;
		}

		self.mac_to_ir(config);
		self.mac_to_rgb_fifo();
	}

	/// Interpolate between a vector and the far color
	fn cmd_intpl(&mut self, config: CommandConfig) {
		let fc = ControlVector::FarColor.index();

		// XXX this is very similar to the loop in DPCS above, we
		// could probably factor that.
		for i in 0..3 {
			let fc = (self.control_vectors[fc][i] as i64) << 12;
			let ir = (self.ir[i + 1] as i64) << 12;

			let sub = fc - ir;

			let tmp = (self.i64_to_i44(i as u8, sub) >> config.shift) as i32;

			let ir0 = self.ir[0] as i64;

			let sat = self.i32_to_i16_saturate(CommandConfig::from_command(0),
											   i as u8, tmp) as i64;

			let res = self.i64_to_i44(i as u8, ir + ir0 * sat);

			self.mac[i + 1] = (res >> config.shift) as i32;
		}

		self.mac_to_ir(config);
		self.mac_to_rgb_fifo();
	}

	/// Multiply vector by matrix and add vector
	fn cmd_mvmva(&mut self, config: CommandConfig) {
		// The fourth vector holds IR values
		self.v[3][0] = self.ir[1];
		self.v[3][1] = self.ir[2];
		self.v[3][2] = self.ir[3];

		self.multiply_matrix_by_vector(config,
									   config.matrix,
									   config.vector_mul,
									   config.vector_add);
	}

	/// Normal Color Depth Cue Single vector
	fn cmd_ncds(&mut self, config: CommandConfig) {
		self.do_ncd(config, 0);
	}

	/// Normal Color Depth Cue Triple
	fn cmd_ncdt(&mut self, config: CommandConfig) {
		self.do_ncd(config, 0);
		self.do_ncd(config, 1);
		self.do_ncd(config, 2);
	}

	/// Normal Color Color Single. Operates on V0
	fn cmd_nccs(&mut self, config: CommandConfig) {
		self.do_ncc(config, 0);
	}

	/// Color Color
	fn cmd_cc(&mut self, config: CommandConfig) {
		self.v[3][0] = self.ir[1];
		self.v[3][1] = self.ir[2];
		self.v[3][2] = self.ir[3];

		self.multiply_matrix_by_vector(config,
									   Matrix::Color,
									   3,
									   ControlVector::BackgroundColor);


		let (r, g, b, _) = self.rgb;

		let col = [r, g, b];

		for i in 0..3 {
			let ir = self.ir[i + 1] as i32;
			let col = (col[i] as i32) << 4;

			self.mac[i + 1] = (ir * col) >> config.shift;
		}

		self.mac_to_ir(config);
		self.mac_to_rgb_fifo();
	}

	fn cmd_ncs(&mut self, config: CommandConfig) {
		self.do_nc(config, 0);
	}

	fn cmd_nct(&mut self, config: CommandConfig) {
		self.do_nc(config, 0);
		self.do_nc(config, 1);
		self.do_nc(config, 2);
	}

	fn cmd_sqr(&mut self, config: CommandConfig) {
		for i in 1..4 {
			let ir = self.ir[i] as i32;

			self.mac[i] = (ir * ir) >> config.shift;
		}

		self.mac_to_ir(config);
	}

	fn cmd_avsz3(&mut self) {
		let z1 = self.z_fifo[1] as u32;
		let z2 = self.z_fifo[2] as u32;
		let z3 = self.z_fifo[3] as u32;

		let sum = z1 + z2 + z3;


		let zsf3 = self.zsf3 as i64;

		let average = zsf3 * sum as i64;

		self.check_mac_overflow(average);

		self.mac[0] = average as i32;
		self.otz = self.i64_to_otz(average);
	}

	fn cmd_avsz4(&mut self) {
		let z0 = self.z_fifo[0] as u32;
		let z1 = self.z_fifo[1] as u32;
		let z2 = self.z_fifo[2] as u32;
		let z3 = self.z_fifo[3] as u32;

		let sum = z0 + z1 + z2 + z3;

		let zsf4 = self.zsf4 as i64;

		let average = zsf4 * sum as i64;

		self.check_mac_overflow(average);

		self.mac[0] = average as i32;
		self.otz = self.i64_to_otz(average);
	}

	fn cmd_rtpt(&mut self, config: CommandConfig) {

		// Transform the three vectors at once
		self.do_rtp(config, 0);
		self.do_rtp(config, 1);
		// We do depth queuing on the last vector
		let projection_factor = self.do_rtp(config, 2);

		self.depth_queuing(projection_factor);
	}

	fn cmd_gpf(&mut self, config: CommandConfig) {
		let ir0 = self.ir[0] as i32;

		for i in 1..4 {
			let ir = self.ir[i] as i32;

			self.mac[i] = (ir * ir0) >> config.shift;
		}

		self.mac_to_ir(config);
		self.mac_to_rgb_fifo();
	}

	fn cmd_gpl(&mut self, config: CommandConfig) {
		let ir0 = self.ir[0] as i32;

		let shift = config.shift;

		for i in 1..4 {
			let ir = self.ir[i] as i32;

			let ir_prod = (ir * ir0) as i64;

			let mac = (self.mac[i] as i64) << shift;

			let sum = self.i64_to_i44((i - 1) as u8, mac + ir_prod);

			self.mac[i] = (sum >> shift) as i32;
		}

		self.mac_to_ir(config);
		self.mac_to_rgb_fifo();
	}

	fn cmd_ncct(&mut self, config: CommandConfig) {

		// Transform the three vectors at once
		self.do_ncc(config, 0);
		self.do_ncc(config, 1);
		self.do_ncc(config, 2);
	}

	fn do_ncc(&mut self, config: CommandConfig, vector_index: u8) {
		self.multiply_matrix_by_vector(config,
									   Matrix::Light,
									   vector_index,
									   ControlVector::Zero);

		// Use the 4th vector to store the intermediate values
		self.v[3][0] = self.ir[1];
		self.v[3][1] = self.ir[2];
		self.v[3][2] = self.ir[3];

		self.multiply_matrix_by_vector(config,
									   Matrix::Color,
									   3,
									   ControlVector::BackgroundColor);


		let (r, g, b, _) = self.rgb;

		let col = [r, g, b];

		for i in 0..3 {
			let col = (col[i] as i32) << 4;
			let ir = self.ir[i + 1] as i32;

			self.mac[i + 1] = (col * ir) >> config.shift;
		}

		self.mac_to_ir(config);
		self.mac_to_rgb_fifo();
	}

	fn do_nc(&mut self, config: CommandConfig, vector_index: u8) {
		self.multiply_matrix_by_vector(config,
									   Matrix::Light,
									   vector_index,
									   ControlVector::Zero);

		self.v[3][0] = self.ir[1];
		self.v[3][1] = self.ir[2];
		self.v[3][2] = self.ir[3];

		self.multiply_matrix_by_vector(config,
									   Matrix::Color,
									   3,
									   ControlVector::BackgroundColor);

		self.mac_to_rgb_fifo();
	}

	fn do_ncd(&mut self, config: CommandConfig, vector_index: u8) {

		self.multiply_matrix_by_vector(config,
									   Matrix::Light,
									   vector_index,
									   ControlVector::Zero);

		self.v[3][0] = self.ir[1];
		self.v[3][1] = self.ir[2];
		self.v[3][2] = self.ir[3];

		self.multiply_matrix_by_vector(config,
									   Matrix::Color,
									   3,
									   ControlVector::BackgroundColor);

		self.cmd_dcpl(config);
	}

	fn multiply_matrix_by_vector(&mut self,
								 config: CommandConfig,
								 matrix: Matrix,
								 vector_index: u8,
								 control_vector: ControlVector) {

		let vector_index = vector_index as usize;

		if matrix == Matrix::Invalid {
			panic!("GTE multiplication with invalid matrix");
		}

		if control_vector == ControlVector::FarColor {
			panic!("GTE multiplication with far color vector");
		}

		let mat = matrix.index();
		let crv = control_vector.index();

		for r in 0..3 {
			let mut res = (self.control_vectors[crv][r] as i64) << 12;
			for c in 0..3 {
				let v = self.v[vector_index][c] as i32;
				let m = self.matrices[mat][r][c] as i32;

				let product = v * m;
				res = self.i64_to_i44(r as u8, res + product as i64);
			}

			self.mac[r + 1] = (res >> config.shift) as i32;
		}

		self.mac_to_ir(config);
	}

	fn mac_to_ir(&mut self, config: CommandConfig) {
		let mac1 = self.mac[1];
		self.ir[1] = self.i32_to_i16_saturate(config, 0, mac1);

		let mac2 = self.mac[2];
		self.ir[2] = self.i32_to_i16_saturate(config, 1, mac2);

		let mac3 = self.mac[3];
		self.ir[3] = self.i32_to_i16_saturate(config, 2, mac3);
	}

	fn mac_to_rgb_fifo(&mut self) {
		fn mac_to_color(gte: &mut Gte, mac: i32, which: u8) -> u8 {
			let c = mac >> 4;

			if c < 0 {
				gte.set_flag(21 - which);
				0
			} else if c > 0xff {
				gte.set_flag(21 - which);
				0xff
			} else {
				c as u8
			}
		}

		let mac1 = self.mac[1];
		let mac2 = self.mac[2];
		let mac3 = self.mac[3];

		let r = mac_to_color(self, mac1, 0);
		let g = mac_to_color(self, mac2, 1);
		let b = mac_to_color(self, mac3, 2);

		let (_, _, _, x) = self.rgb;

		self.rgb_fifo[0] = self.rgb_fifo[1];
		self.rgb_fifo[1] = self.rgb_fifo[2];
		self.rgb_fifo[2] = (r, g, b, x);
	}

	fn do_rtp(&mut self, config: CommandConfig, vector_index: usize) -> u32 {
		let mut z_shifted: i32 = 0;
		let rm = Matrix::Rotation.index();
		let tr = ControlVector::Translation.index();

		for r in 0..3 {
			let mut res = (self.control_vectors[tr][r] as i64) << 12;

			for c in 0..3 {
				let v = self.v[vector_index][c] as i32;
				let m = self.matrices[rm][r][c] as i32;

				let rot = v * m;

				res = self.i64_to_i44(c as u8, res + rot as i64);
			}

			self.mac[r + 1] = (res >> config.shift) as i32;

			z_shifted = (res >> 12) as i32;
		}

		let val = self.mac[1];
		self.ir[1] = self.i32_to_i16_saturate(config, 0, val);
		let val = self.mac[2];
		self.ir[2] = self.i32_to_i16_saturate(config, 1, val);

		let min = i16::MIN as i32;
		let max = i16::MAX as i32;

		if z_shifted > max || z_shifted < min {
			self.set_flag(22);
		}

		let min =
			match config.clamp_negative {
				true  => 0,
				false => i16::MIN as i32,
			};

		let val = self.mac[3];

		self.ir[3] =
			if val < min {
				min as i16
			} else if val > max {
				max as i16
			} else {
				val as i16
			};

		let z_saturated =
			if z_shifted < 0 {
				self.set_flag(18);
				0
			} else if z_shifted > u16::MAX as i32 {
				self.set_flag(18);
				u16::MAX
			} else {
				z_shifted as u16
			};

		self.z_fifo[0] = self.z_fifo[1];
		self.z_fifo[1] = self.z_fifo[2];
		self.z_fifo[2] = self.z_fifo[3];
		self.z_fifo[3] = z_saturated;

		let projection_factor =
			if z_saturated > self.h / 2 {
				divider::divide(self.h, z_saturated)
			} else {
				self.set_flag(17);

				0x1ffff
			};

		let factor = projection_factor as i64;
		let x = self.ir[1] as i64;
		let y = self.ir[2] as i64;
		let ofx = self.ofx as i64;
		let ofy = self.ofy as i64;

		let screen_x = x * factor + ofx;
		let screen_y = y * factor + ofy;

		self.check_mac_overflow(screen_x);
		self.check_mac_overflow(screen_y);

		let screen_x = (screen_x >> 16) as i32;
		let screen_y = (screen_y >> 16) as i32;

		self.xy_fifo[3] = (self.i32_to_i11_saturate(0, screen_x),
						   self.i32_to_i11_saturate(1, screen_y));

		self.xy_fifo[0] = self.xy_fifo[1];
		self.xy_fifo[1] = self.xy_fifo[2];
		self.xy_fifo[2] = self.xy_fifo[3];

		projection_factor
	}

	fn depth_queuing(&mut self, projection_factor: u32) {
		let factor = projection_factor as i64;
		let dqa = self.dqa as i64;
		let dqb = self.dqb as i64;

		let depth = dqb + dqa * factor;

		self.check_mac_overflow(depth);

		self.mac[0] = depth as i32;

		let depth = depth >> 12;

		self.ir[0] =
			if depth < 0 {
				self.set_flag(12);
				0
			} else if depth > 4096 {
				self.set_flag(12);
				4096
			} else {
				depth as i16
			};
	}

	fn set_flag(&mut self, bit: u8) {
		self.flags |= 1 << bit;
	}

	fn i64_to_i44(&mut self, flag: u8, val: i64) -> i64 {
		if val > 0x7ffffffffff {
			self.set_flag(30 - flag);
		} else if val < -0x80000000000 {
			self.set_flag(27 - flag);
		}

		(val << (64 - 44)) >> (64 - 44)
	}

	fn i32_to_i16_saturate(&mut self,
						   config: CommandConfig,
						   flag: u8,
						   val: i32) -> i16 {
		let min =
			match config.clamp_negative {
				true  => 0,
				false => i16::MIN as i32,
			};

		let max = i16::MAX as i32;

		if val > max {
			self.set_flag(24 - flag);
			max as i16
		} else if val < min {
			self.set_flag(24 - flag);

			min as i16
		} else {
			val as i16
		}
	}

	fn i32_to_i11_saturate(&mut self, flag: u8, val: i32) -> i16 {
		if val < -0x400 {
			self.set_flag(14 - flag);
			-0x400
		} else if val > 0x3ff {
			self.set_flag(14 - flag);
			0x3ff
		} else {
			val as i16
		}
	}

	fn check_mac_overflow(&mut self, val: i64) {
		if val < -0x80000000 {
			self.set_flag(15);
		} else if val > 0x7fffffff {
			self.set_flag(16);
		}
	}

	fn i64_to_otz(&mut self, average: i64) -> u16 {
		let value = average >> 12;

		if value < 0 {
			self.set_flag(18);
			0
		} else if value > 0xffff {
			self.set_flag(18);
			0xffff
		} else {
			value as u16
		}
	}
}


#[derive(Clone, Copy)]
struct CommandConfig {
	shift: u8,
	clamp_negative: bool,
	matrix: Matrix,
	vector_mul: u8,
	vector_add: ControlVector,
}

impl CommandConfig {
	fn from_command(command: u32) -> CommandConfig {
		let shift =
			if command & (1 << 19) != 0 {
				12
			} else {
				0
			};
		let clamp_negative = command & (1 << 10) != 0;
		let vector_index = (command >> 15) & 3;
		CommandConfig {
			shift,
			clamp_negative,
			matrix: Matrix::from_command(command),
			vector_mul: vector_index as u8,
			vector_add: ControlVector::from_command(command),
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Matrix {
	Rotation = 0,
	Light = 1,
	Color = 2,
	Invalid,
}

impl Matrix {
	fn from_command(command: u32) -> Matrix {
		match (command >> 17) & 3 {
			0 => Matrix::Rotation,
			1 => Matrix::Light,
			2 => Matrix::Color,
			3 => Matrix::Invalid,
			_ => unreachable!(),
		}
	}
	fn index(self) -> usize {
		self as usize
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlVector {
	Translation = 0,
	BackgroundColor = 1,
	FarColor = 2,
	Zero = 3,
}

impl ControlVector {
	fn from_command(command: u32) -> ControlVector {
		match (command >> 13) & 3 {
			0 => ControlVector::Translation,
			1 => ControlVector::BackgroundColor,
			2 => ControlVector::FarColor,
			3 => ControlVector::Zero,
			_ => unreachable!(),
		}
	}
	fn index(self) -> usize {
		self as usize
	}
}