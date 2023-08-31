mod decode;
mod encode;
mod common;

pub mod prelude {
	pub use super::decode::DecoderState;
	pub use super::encode::EncoderState;
}

#[cfg(test)]
mod tests {
	use super::prelude::*;

	#[test]
	fn test_round_trip_1() {
		let bytes = vec![0xFF, 0x10, 0x00];

		let mut encoder: EncoderState<u8> = EncoderState::default();
		let data_encoded = encoder.push_slice(&bytes);

		let mut decoder = DecoderState::new(bytes.len());
		decoder.push_slice(&data_encoded);
		let output = decoder.read();

		assert_eq!(bytes, output);
	}

	#[test]
	fn round_trip_all_3_bit_sequences() {
		let bytes = vec![
			0b11110000,
			0b11001100,
			0b10101010,
		];

		let mut encoder: EncoderState<u8> = EncoderState::default();
		let data_encoded = encoder.push_slice(&bytes);

		let mut decoder = DecoderState::new(bytes.len());
		decoder.push_slice(&data_encoded);
		dbg!(&decoder.decoders[0]);
		let output = decoder.read();

		assert_eq!(bytes, output);
	}
}
