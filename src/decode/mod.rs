mod single_bit_decode;

use single_bit_decode::BitDecoderState;
use crate::common::*;

#[derive(Debug)]
pub struct DecoderState {
	decoders: [BitDecoderState; 8]
}

impl DecoderState {
	pub fn new(len: usize) -> Self {
		Self {
			decoders: [
				BitDecoderState::new(len),
				BitDecoderState::new(len),
				BitDecoderState::new(len),
				BitDecoderState::new(len),
				BitDecoderState::new(len),
				BitDecoderState::new(len),
				BitDecoderState::new(len),
				BitDecoderState::new(len)
			]
		}
	}

	pub fn push(&mut self, byte0: u8, byte1: u8) {
		for i in 0..8 {
			self.decoders[i].push(byte1 & BIT_MASK[i], byte0 & BIT_MASK[i])
		}
	}

	pub fn read(self) -> Vec<u8> {
		let mut ans = self.decoders[0].read(BIT_MASK[0]);

		for x in 1..8 {
			let new = self.decoders[x].read(BIT_MASK[x]);

			for i in 0..ans.len() {
				ans[i] |= new[i];
			}
		}

		ans
	}
}