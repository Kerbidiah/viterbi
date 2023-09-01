use crate::common::*;
use crate::encode::EncoderState;

#[derive(Debug)]
pub struct BitDecoderState {
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
	pub fn push(&mut self, s0: u8, s1: u8) {
		let bit_pair = combine(s0, s1);

		for state in self.states() {
			for (link, pos) in Link::next(state, bit_pair, self.prev_cost(state)) {
				self.add_link(link, pos);
			}
		}

		self.level += 1;
	}

	/// ouputs a vector of u8s where only the correct bits are set to 1
	pub fn read(&self, bit: u8) -> Vec<u8> {
		// TODO: figure out what to do if the decoder isn't yet full
		// for now, just assert that it is
		assert!(self.trellis.last().unwrap()[0].is_some());

		let mut ans = Vec::with_capacity(self.level as usize);

		let last_index = self.level - 1;

		// find the link to start from
		let mut min_cost_state = 0;
		let mut min_cost = self.get_any_link(last_index, min_cost_state).unwrap().cost;

		for x in 1..4 {
			let current_cost = self.get_any_link(last_index, x).unwrap().cost;
			if current_cost < min_cost {
				min_cost = current_cost;
				min_cost_state = x;
			}
		}

		// follow the links to the start and record what bit we think was encoded
		for offset in 0..=last_index {
			let i = last_index - offset; // index from end to start

			ans.push(map_to(min_cost_state & BIT_MASK[0], bit));

			min_cost_state = self.get_any_link(i, min_cost_state).unwrap().prev_state;
		}

		ans
	}

	fn states(&self) -> Vec<u8> {
		match self.level {
			0 => vec![0],
			1 => vec![0, 1],
			_ => vec![0, 1, 2, 3]
		}
	}

	fn get_any_link_mut(&mut self, level: u8, pos: u8) -> &mut Option<Link> {
		&mut self.trellis[level as usize][pos as usize]
	}

	fn get_any_link(&self, level: u8, pos: u8) -> Option<Link> {
		self.trellis[level as usize][pos as usize]
	}

	fn get_link_mut(&mut self, pos: u8) -> &mut Option<Link> {
		self.get_any_link_mut(self.level, pos)
	}

	fn add_link(&mut self, new_link: Link, pos: u8) {
		if let Some(mut current_link) = self.get_link_mut(pos) {
			current_link.minimize_cost(new_link);
		} else {
			*self.get_link_mut(pos) = Some(new_link);
		}
	}

	#[inline]
	fn prev_cost(&self, state: u8) -> u8 {
		if self.level == 0 {
			0
		} else {
			self.get_any_link(self.level - 1, state).unwrap().cost
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Link {
	pub prev_state: u8,

	/// enough for the decoder to consume 254 bits (yeilding 127 bits) no matter what
	pub cost: u8,
}

impl Link {
	/// return the next 2 links and where the link should be placed
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

		let mut encoder: EncoderState<u8> = state.into();
		let hypothetical_bit_pair = encoder.push_return_bitpair(stretch(bit));

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
		assert_eq!(Link::hamming_dist(2, 0), 1);
		assert_eq!(Link::hamming_dist(2, 1), 2);
		assert_eq!(Link::hamming_dist(0, 0), 0);
		assert_eq!(Link::hamming_dist(255, 255), 0);
		assert_eq!(Link::hamming_dist(0b00010101, 0b00000100), 2);
	}

	#[test]
	fn test_minimize_cost() {
		let mut link_a = Link {
			prev_state: 0,
			cost: 10
		};

		let link_b = Link {
			prev_state: 1,
			cost: 11
		};

		link_a.minimize_cost(link_b);

		assert_eq!(link_a.prev_state, 0);
		assert_eq!(link_a.cost, 10);

		let link_c = Link {
			prev_state: 2,
			cost: 9,
		};

		link_a.minimize_cost(link_c);

		assert_eq!(link_a.prev_state, 2);
		assert_eq!(link_a.cost, 9);
	}

	#[test]
	fn test_next_link() {
		let arr = Link::next(1, 2, 0);

		assert_eq!(arr[0].0, Link {
			prev_state: 1,
			cost: 0
		});

		assert_eq!(arr[1].0, Link {
			prev_state: 1,
			cost: 2
		});
	}

	#[test]
	fn test_generate_link() {
		let (link_0, _) = Link::generate(1, 2, 0, 1);

		assert_eq!(link_0, Link {
			prev_state: 1,
			cost: 2
		});
	}
}
