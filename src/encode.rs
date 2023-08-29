#[derive(Debug, Clone)]
/// represents the internal state of 8 encoders. (each bit of a byte has its own encoder, this should make the encoder *much* more efficent)
/// I think this could even be extended to like u64 or even u128 if we wanted to
/// 
/// for more detail on how this works see [https://youtu.be/kRIfpmiMCpU?list=PLvJZZcg6Js7oiWPv5XVBXQp8vjffJkz5W](this)
pub struct EncoderState(u8, u8);

impl Default for EncoderState {
	fn default() -> Self {
		Self(0, 0)
	}
}

impl EncoderState {
	/// input a byte to the encoder
	pub fn input(&mut self, byte: u8) -> (u8, u8) {
		let ans = (
			self.1 ^ byte,
			self.0 ^ self.1 ^ byte
		);

		self.update(byte);

		ans
	}

	#[inline]
	fn shift(&mut self) {
		self.1 = self.0;
	}

	#[inline]
	/// update the state.
	fn update(&mut self, byte: u8) {
		self.shift();
		self.0 = byte;
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

	fn eq(state: &EncoderState, x: u8) {
		let (a, b) = convert(x);
		assert_eq!(state.0, a);
		assert_eq!(state.1, b);
	}

	#[test]
	fn test_state_updating() {
		let mut state = EncoderState::default();

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
