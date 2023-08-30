use crate::common::*;
use crate::encode::EncoderState;

#[derive(Debug)]
struct BitDecoderState {
	trellis: Vec<[Option<Link>; 4]>, // TODO: remove option? it would make the code less nice, but its not actually doing much
	level: u8
}

impl BitDecoderState {
	pub fn new(len: usize) -> Self {
		assert!(len <= 127);
		assert!(len >= 2); // idk if this is needed

		Self {
			trellis: vec![[None; 4]; len],
			level: 0
		}
	}

	/// push a pair of bits to be decoded
	/// 
	/// takes u8s instead of bools for conveince (just do a `bitwise and` between the mask and the byte)
	pub fn push(&mut self, s1: u8, s0: u8) {
		let bit_pair = combine(s1, s0);
		
		for state in self.states() {
			for (link, pos) in Link::next(state, bit_pair, self.prev_cost(state)) {
				self.add_link(link, pos);
			}
		}

		self.level += 1;
	}

	fn states(&self) -> Vec<u8> {
		match self.level {
			0 => vec![0], // i don't think this one is used
			1 => vec![0, 1],
			_ => vec![0, 1, 2, 3]
		}
	}

	fn add_link(&mut self, new_link: Link, pos: u8) {
		if let Some(mut current_link) = self.trellis[self.level as usize][pos as usize] {
			current_link.minimize_cost(new_link);
		} else {
			self.trellis[self.level as usize][pos as usize] = Some(new_link)
		}
	}

	#[inline]
	fn prev_cost(&self, state: u8) -> u8 {
		if self.level == 0 {
			0
		} else {
			self.trellis[(self.level - 1) as usize][state as usize].unwrap().cost
		}
	}
}

#[derive(Debug, Clone, Copy)]
struct Link {
	pub prev_state: u8,

	/// enough for the decoder to consume 254 bits (yeilding 127 bits) no matter what
	pub cost: u8,
}

impl Link {
	/// return the correct 2 links and where the link should be placed
	pub fn next(state: u8, bit_pair: u8, prev_cost: u8) -> [(Self, u8); 2] {
		[
			Self::generate(state, bit_pair, prev_cost, 0),
			Self::generate(state, bit_pair, prev_cost, 1)
		]
	}

	pub fn minimize_cost(&mut self, other: Self) {
		// TODO: figure out what should be done if the costs are the same...
		if self.cost > other.cost {
			*self = other;
		}
	}

	fn generate(state: u8, bit_pair: u8, prev_cost: u8, bit: u8)  -> (Self, u8) {
		/* NOTES to self
		* the prev_state for each link is simply the state parameter
		* hamming dist is between bit_pair and what comes out of the encoder.input_byte_out function
		* the correct placement for each link is the internal state of its encoder after inputting the 1 or 0
		 */
		
		debug_assert!(bit <= 1); // I don't know if this is actually needed, but it can't hurt

		let mut encoder: EncoderState<u8> = state.into();
		let hypothetical_bit_pair = encoder.input_w_bitpair_return(bit);

		(
			Self {
				prev_state: state,
				cost: prev_cost + Self::hamming_dist(bit_pair, hypothetical_bit_pair)
			},
			encoder.into()
		)
	}

	fn hamming_dist(a: u8, b: u8) -> u8 {
		(a ^ b).count_ones() as u8
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hamming_distance() {
		assert_eq!(Link::hamming_dist(255, 0), 8);
		assert_eq!(Link::hamming_dist(1, 0), 1);
		assert_eq!(Link::hamming_dist(0, 0), 0);
		assert_eq!(Link::hamming_dist(255, 255), 0);
		assert_eq!(Link::hamming_dist(0b00010101, 0b00000100), 2);
	}
}
