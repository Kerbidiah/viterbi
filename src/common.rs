#[inline]
/// Any input >= 1 becomes 1
fn squish(num: u8) -> u8 {
	match num {
		0 => 0,
		_ => 1,
	}
}


#[inline]
/// Any input >= 1 becomes 255 (aka `0b11111111`)
fn stretch(num: u8) -> u8 {
	match num {
		0 => 0,
		_ => 0xFF,
	}
}

#[inline]
/// combine s1 and s0 into the lower 2 bits of a u8
pub fn combine(s1: u8, s0: u8) -> u8 {
	(squish(s1) << 1) | squish(s0)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_convert() {
		assert_eq!(squish(0), 0);

		for i in 1u8..=255 {
			assert_eq!(squish(i), 1);
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