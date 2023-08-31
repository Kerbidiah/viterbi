use std::ops::BitXor;

use crate::common::*;

#[derive(Debug, Clone, Default)]
/// represents the internal state of multiple encoders. (each bit is its own encoder)
/// 
/// for more detail on how this works see [this video](https://youtu.be/kRIfpmiMCpU)
pub struct EncoderState<T: BitXor + Copy>(T, T);

impl<T: BitXor<Output = T> + Copy> EncoderState<T> {
	/// input a chunk to the encoder, updating state and returning the 2 chunks that should be transmitted
	pub fn push(&mut self, chunk: T) -> (T, T) {
		let ans = (
			self.1 ^ chunk,
			self.0 ^ self.1 ^ chunk
		);

		self.update(chunk);

		ans
	}

	#[inline]
	fn shift(&mut self) {
		self.1 = self.0;
	}

	#[inline]
	/// update the state.
	fn update(&mut self, chunk: T) {
		self.shift();
		self.0 = chunk;
	}
}

impl From<u8> for EncoderState<u8> {
	fn from(value: u8) -> Self {
		match value {
			0b00 => Self(0x00, 0x00),
			0b01 => Self(0xFF, 0x00),
			0b10 => Self(0x00, 0xFF),
			0b11 => Self(0xFF, 0xFF),
			_ => unreachable!()
		}
	}
}

impl From<EncoderState<u8>> for u8 {
	fn from(value: EncoderState<u8>) -> Self {
		combine(value.1, value.0)
	}
}

impl EncoderState<u8> {
	// TODO: rename this function. Its name is so bad
	/// does the same thing as input, but it combines the 2 bytes into a bit pair
	/// 
	/// NOTE: this won't work in a usefull manner if you are using the EncoderState to encode multiple bits side by side
	/// its only purpose really is for testing
	pub fn input_w_bitpair_return(&mut self, byte: u8) -> u8 {
		let (s0, s1) = self.push(byte);
		combine(s1, s0)
	}

	// fn push_return_bytepair(&mut self, byte: u8) -> [u8; 2] {

	// }

	pub fn push_slice(&mut self, arr: &[u8]) -> Vec<u8> {
		let mut ans = Vec::with_capacity(arr.len() * 2);

		for each in arr {

		}

		ans
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn convert(x: u8) -> (u8, u8) {
		match x {
			0b00 => (0x00, 0x00),
			0b01 => (0xFF, 0x00),
			0b10 => (0x00, 0xFF),
			0b11 => (0xFF, 0xFF),
			_ => unreachable!()
		}
	}

	fn eq(state: &EncoderState<u8>, x: u8) {
		let (a, b) = convert(x);
		assert_eq!(state.0, a);
		assert_eq!(state.1, b);
	}

	#[test]
	fn test_state_updating() {
		let mut state = EncoderState::<u8>::default();

		state.update(0x00);
		eq(&state, 0b00);
		state.update(0xFF);
		eq(&state, 0b01);
		state.update(0x00);
		eq(&state, 0b10);

		state = EncoderState(0xFF, 0x00);
		state.update(0xFF);
		eq(&state, 0b11);
		state.update(0xFF);
		eq(&state, 0b11);
		state.update(0x00);
		eq(&state, 0b10);
		state.update(0xFF);
		eq(&state, 0b01);

		state = EncoderState(0x00, 0xFF);
		state.update(0x00);
		eq(&state, 0b00);
	}

	#[test]
	fn test_to_from_encoder_state() {
		for x in 0u8..4 {
			let state: EncoderState<u8> = x.into();
			assert_eq!(x, dbg!(state.into()));
		}
	}
	
	#[test]
	fn test_to_encoder_state() {
		let mut state: EncoderState<u8> = 0b10.into();
		assert_eq!(state.1, 0xFF);

		state = 0b01.into();
		assert_eq!(state.0, 0xFF);
	}
}
