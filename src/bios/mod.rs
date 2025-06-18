
pub mod db;

use std::{fmt, marker::PhantomData};


use serde::{de::{SeqAccess, Visitor}, ser::SerializeSeq, Deserialize, Deserializer, Serialize};

use crate::{box_array, cdrom::disk::Region, memory::Addressable};


pub const BIOS_SIZE: usize = 512 * 1024;

use self::db::Metadata;



pub struct Bios {
	data: Box<[u8; BIOS_SIZE]>,
	metadata:	&'static Metadata
}

impl Bios {
	pub fn new(binary: Box<[u8; BIOS_SIZE]>) -> Option<Bios> {
		match db::lookup_blob(&*binary) {
			Some(metadata) => Some(Bios {
				data: binary,
				metadata: metadata
			}),
			None => None
		}
	}

	pub fn dummy() -> Bios {
		let mut bios =
			Bios {
				data: box_array![0; BIOS_SIZE],
				metadata: &DUMMY_METADATA,
			};

		for (i, b) in bios.data.iter_mut().enumerate() {
			*b = (0x7BADB105 >> ((i % 4) * 2)) as u8;
		}
		
		bios
	}

	pub fn patch_boot_animation(&mut self) -> Result<(), ()> {
		self.patch_animation_jump_hook(0)
	}

	pub fn patch_animation_jump_hook(&mut self, instruction: u32) -> Result<(), ()> {
		match self.metadata.animation_jump_hook {
			Some(h) => {
				let h = h as usize;
				self.data[h] = instruction as u8;
				self.data[h + 1] = (instruction >> 8) as u8;
				self.data[h + 2] = (instruction >> 16) as u8;
				self.data[h + 3] = (instruction >> 24) as u8;
				Ok(())
			}
			None => Err(())
		}
	}

	pub fn enable_debug_uart(&mut self) -> Result<(), ()> {
        match self.metadata.patch_debug_uart {
            Some(patch) => {
                patch(self);
                Ok(())
            },
            None => Err(()),
        }
    }

	pub fn load<T: Addressable>(&self, offset: u32) -> u32 {
        let offset = offset as usize;

        let mut r = 0;

        for i in 0..T::size() as usize {
            r |= (self.data[offset + i] as u32) << (8 * i)
        }

        r
    }

    pub fn metadata(&self) -> &'static Metadata {
        self.metadata
    }
}

impl Serialize for Bios {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer 
	{
		let sha256 = &self.metadata.sha256;
		let mut seq = serializer.serialize_seq(Some(sha256.len()))?;
		for (i, b) in sha256.iter().enumerate() {
			seq.serialize_element(b)?;
		}
		seq.end()
	}
}

impl<'de> Deserialize<'de> for Bios {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: serde::Deserializer<'de>
	{
		

		deserializer.deserialize_seq()	
	}
}


fn decode_bios<'de, D>(deserialize: D) -> Result<Bios, D::Error>
where 
	D: Deserializer<'de>,
{
	struct BiosParser;
	impl<'de> Visitor<'de> for BiosParser
	{
		type Value = Self;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("[u64; 32]")
		}

		fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
			
			
			

			let mut bios = Bios::dummy();



		}
	}
}

static DUMMY_METADATA: Metadata =
    Metadata {
        sha256: [0xff; 32],
        version_major: 0,
        version_minor: 0,
        region: Region::NorthAmerica,
        known_bad: true,
        animation_jump_hook: None,
        patch_debug_uart: None,
    };