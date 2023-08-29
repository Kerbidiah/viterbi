use std::ops::BitXor;

#[derive(Debug, Clone, Default)]
/// represents the internal state of multiple encoders. (each bit is its own encoder)
/// 
/// for more detail on how this works see [this video](https://youtu.be/kRIfpmiMCpU)
pub struct EncoderState<T: BitXor + Copy>(T, T);

impl<T: BitXor<Output = T> + Copy> EncoderState<T> {
	/// input a chunk to the encoder, updating state and returning the 2 chunks that should be transmitted
	pub fn input(&mut self, chunk: T) -> (T, T) {
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

#[cfg(test)]
mod tests {
	use super::*;

	fn convert(x: u8) -> (u8, u8) {
		match x {
			0b00 => (0x00, 0x00),
			0b01 => (0xFF, 0x00),
			0b10 => (0x00, 0xFF),
			0b11 => (0xFF, 0xFF),
			_ => unimplemented!()
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
}
