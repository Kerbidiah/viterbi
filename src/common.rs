#![allow(dead_code)]

#[inline]
/// combine s1 and s0 into the lower 2 bits of a u8
pub fn combine(s1: u8, s0: u8) -> u8 {
	(squish(s1) << 1) | squish(s0)
}

pub const BIT_MASK: [u8; 8] = [
	1,
	1 << 1,
	1 << 2,
	1 << 3,
	1 << 4,
	1 << 5,
	1 << 6,
	1 << 7
];

#[inline]
/// Any input >= 1 becomes the desired u8
pub fn map_to(num: u8, desired: u8) -> u8 {
	match num {
		0 => 0,
		_ => desired,
	}
}

#[inline]
/// Any input >= 1 becomes 1
pub fn squish(num: u8) -> u8 {
	map_to(num, 1)
}

#[inline]
/// Any input >= 1 becomes 255 (aka `0b11111111`)
pub fn stretch(num: u8) -> u8 {
	map_to(num, 0xFF)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_squish() {
		assert_eq!(squish(0), 0);

		for i in 1u8..=255 {
			assert_eq!(squish(i), 1);
		}
	}

	#[test]
	fn test_stretch() {
		assert_eq!(stretch(0), 0);

		for i in 1u8..=255 {
			assert_eq!(stretch(i), 0xFF);
		}
	}

	#[test]
	fn test_combine() {
		assert_eq!(combine(0, 0), 0b00);
		assert_eq!(combine(0, 255), 0b01);
		assert_eq!(combine(1, 0), 0b10);
		assert_eq!(combine(100, 2), 0b11);
	}
}