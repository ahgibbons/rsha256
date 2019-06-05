use std::fs;
use std::io::Read;
use std::io;
use std::fs::File;
use std::num::Wrapping;

mod shafuncs;

static CHUNKSIZE_BYTES: usize = 64;

fn main() {
	let filename = "abc.txt";

	let mut f = File::open(filename).expect("Could not open file.");

	let mut buffer: [u8; 64] = [0; 64];

	let mut digest: [Wrapping<u32>; 8]= [Wrapping(0x6a09e667),
	Wrapping(0xbb67ae85),
	Wrapping(0x3c6ef372),
	Wrapping(0xa54ff53a),
	Wrapping(0x510e527f),
	Wrapping(0x9b05688c),
	Wrapping(0x1f83d9ab),
	Wrapping(0x5be0cd19)];

	// declare variables


	let mut readbyte;
	let mut messagelength : u64 = 0;
	while {readbyte = f.read(&mut buffer).expect("Could not read from file!"); readbyte==CHUNKSIZE_BYTES} {
		messagelength += readbyte as u64;
		hashround(digest, buffer)
	}
	messagelength += readbyte as u64;
	shafuncs::padmessage(&mut buffer, readbyte, messagelength);
	hashround(digest, buffer);

	for i in K.iter() {
		println!("{:?}", i);
	}

	println!("");
	for i in K2.iter() {
		println!("{:?}", i);
	}

}



fn hashround(digest: [Wrapping<u32>; 8], bytebuffer: [u8; 64]) {
	let mut hvars: [Wrapping<u32>; 8] = digest.clone();	
	let mut t1: Wrapping<u32>;
	let mut t2: Wrapping<u32>;

	let wordbuffer = bytestowords(bytebuffer);

	let mut a: Wrapping<u32> = digest[0];
	let mut b: Wrapping<u32> = digest[1];
	let mut c: Wrapping<u32> = digest[2];
	let mut d: Wrapping<u32> = digest[3];
	let mut e: Wrapping<u32> = digest[4];
	let mut f: Wrapping<u32> = digest[5];
	let mut g: Wrapping<u32> = digest[6];
	let mut h: Wrapping<u32> = digest[7];

	let w = shafuncs::message_schedule(wordbuffer);

	for t in 0..64 {
		t1 = h + shafuncs::S1(e) + shafuncs::ch(e,f,g) + K[t] + w[t];
		t2 = shafuncs::S0(a) + shafuncs::maj(a,b,c);
		h = g;
		g = f;
		f = e;
		e = d+t1;
		d = c;
		c = b;
		b = a;
		a = t1+t2;
	}

}

fn bytestowords(bytebuffer: [u8; 64]) -> [Wrapping<u32>; 16] {
	let mut wordbuffer: [Wrapping<u32>; 16] = [Wrapping(0);16];
	let mut v: Wrapping<u32>;
	for i in 0..16 {
		v=Wrapping(0);
		v += Wrapping(bytebuffer[4*i] as u32) << (3*8);
		v += Wrapping(bytebuffer[4*i+1] as u32) << (2*8);
		v += Wrapping(bytebuffer[4*i+2] as u32) << (1*8);
		v += Wrapping(bytebuffer[4*i+3] as u32);
		wordbuffer[i] = v;
 	}

 	wordbuffer

}

macro_rules! wraparray {
	( $( $x:expr),* ) => {
		[$(Wrapping($x)),*]
	}
}

const K2: [Wrapping<u32>; 64] = wraparray![

	0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
  	0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
	0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
	0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
	0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
	0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
	0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
	0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
	];

const K: [Wrapping<u32>; 64] = [
	Wrapping(0x428a2f98), Wrapping(0x71374491), Wrapping(0xb5c0fbcf), Wrapping(0xe9b5dba5), Wrapping(0x3956c25b), Wrapping(0x59f111f1), Wrapping(0x923f82a4), Wrapping(0xab1c5ed5),
  	Wrapping(0xd807aa98), Wrapping(0x12835b01), Wrapping(0x243185be), Wrapping(0x550c7dc3), Wrapping(0x72be5d74), Wrapping(0x80deb1fe), Wrapping(0x9bdc06a7), Wrapping(0xc19bf174),
	Wrapping(0xe49b69c1), Wrapping(0xefbe4786), Wrapping(0x0fc19dc6), Wrapping(0x240ca1cc), Wrapping(0x2de92c6f), Wrapping(0x4a7484aa), Wrapping(0x5cb0a9dc), Wrapping(0x76f988da),
	Wrapping(0x983e5152), Wrapping(0xa831c66d), Wrapping(0xb00327c8), Wrapping(0xbf597fc7), Wrapping(0xc6e00bf3), Wrapping(0xd5a79147), Wrapping(0x06ca6351), Wrapping(0x14292967),
	Wrapping(0x27b70a85), Wrapping(0x2e1b2138), Wrapping(0x4d2c6dfc), Wrapping(0x53380d13), Wrapping(0x650a7354), Wrapping(0x766a0abb), Wrapping(0x81c2c92e), Wrapping(0x92722c85),
	Wrapping(0xa2bfe8a1), Wrapping(0xa81a664b), Wrapping(0xc24b8b70), Wrapping(0xc76c51a3), Wrapping(0xd192e819), Wrapping(0xd6990624), Wrapping(0xf40e3585), Wrapping(0x106aa070),
	Wrapping(0x19a4c116), Wrapping(0x1e376c08), Wrapping(0x2748774c), Wrapping(0x34b0bcb5), Wrapping(0x391c0cb3), Wrapping(0x4ed8aa4a), Wrapping(0x5b9cca4f), Wrapping(0x682e6ff3),
	Wrapping(0x748f82ee), Wrapping(0x78a5636f), Wrapping(0x84c87814), Wrapping(0x8cc70208), Wrapping(0x90befffa), Wrapping(0xa4506ceb), Wrapping(0xbef9a3f7), Wrapping(0xc67178f2)
];