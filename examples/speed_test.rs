use std::hint::black_box;
use std::time::Instant;

use clap::Parser;
use bytes::{Bytes, Buf};
use bytesize::ByteSize;
use rand::prelude::*;

use viterbi::prelude::*;

#[derive(Parser)]
#[command()]
struct Cli {
	/// How much data to generate (eg: 1.5KiB, 518.0 GB, 215 B)
	bytes: String,

	/// Use hashing to verify correctness instead of holding onto generated data
	#[arg(short = 'H', long)]
	hashify: bool,

	/// how many bytes should each encoder be fed
	#[arg(short, default_value_t = 127)]
	packet_size: usize
}

fn main() {
	let cli = Cli::parse();

	let ammount = cli.bytes.parse::<ByteSize>().unwrap();

	if (ammount > ByteSize::gib(1)) && !cli.hashify {
		todo!("must use hashify for more than 1 GiB");
	}
	if cli.hashify {
		todo!("hashing not yet implemented");
	}

	let mut ammount = ammount.as_u64() as usize;
	ammount -= ammount % cli.packet_size; // trim ammount slightly so it works nicely

	println!("random data generation starting");
	let data = black_box(random_bytes(ammount));
	println!("data generation complete");
	
	println!("encoding is starting");
	let encoding_timer = Instant::now();
	let transmitted = black_box(encode(data.clone(), cli.packet_size));
	let encoding_time = encoding_timer.elapsed();
	dbg!(encoding_time.as_millis());

	println!("decoding is starting");
	let decoding_timer = Instant::now();
	let output = black_box(decode(transmitted, cli.packet_size));
	let decoding_time = decoding_timer.elapsed();
	dbg!(decoding_time.as_millis());

	assert_eq!(data, output);
}

fn random_bytes(len: usize) -> Bytes {
	let mut rng = rand::thread_rng();
	let mut data: Vec<u8> = vec![0; len];
	rng.fill_bytes(&mut data);

	data.into()
}

fn encode(data: Bytes, packet_len: usize) -> Bytes {
	data.chunks_exact(packet_len).map(|arr| {
		let mut encoder = EncoderState::default();

		encoder.push_slice(&arr)
	}).flatten().collect()
}

fn decode(data: Bytes, packet_len: usize) -> Bytes {
	data.chunks_exact(packet_len * 2).map(|arr| {
		let mut decoder = DecoderState::new(packet_len);
		decoder.push_slice(&arr);

		decoder.read()
	}).flatten().collect()
}
