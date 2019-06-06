use std::io::Read;
use std::fs::File;
use std::num::Wrapping;
use std::env;

mod shafuncs;

macro_rules! wraparray {
	( $( $x:expr),* ) => {
		[$(Wrapping($x)),*]
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("Could not open file.");

	let mut buffer: [u8; 64] = [0; 64];
	let mut readlen;
	let mut messagelength : u64 = 0;

	let mut digest: [Wrapping<u32>; 8]= wraparray![
			0x6a09e667,
			0xbb67ae85,
			0x3c6ef372,
			0xa54ff53a,
			0x510e527f,
			0x9b05688c,
			0x1f83d9ab,
			0x5be0cd19];


	while {readlen = f.read(&mut buffer).expect("Could not read from file!"); readlen==shafuncs::CHUNKBYTES} {
		messagelength += readlen as u64;
		hashround(&mut digest, buffer)
	}
	messagelength += readlen as u64;
	shafuncs::padmessage(&mut buffer, readlen, messagelength);
	hashround(&mut digest, buffer);

	// Final output
	for i in digest.iter() {
		print!("{:08x}", i);
	}
	println!("  {}", filename);
}



fn hashround(digest: &mut [Wrapping<u32>; 8], bytebuffer: [u8; 64]) {
	let mut t1: Wrapping<u32>;
	let mut t2: Wrapping<u32>;

	let wordbuffer = bytestowords(bytebuffer);

	// Message Schedule
	let w = shafuncs::message_schedule(wordbuffer);

	let mut a: Wrapping<u32> = digest[0];
	let mut b: Wrapping<u32> = digest[1];
	let mut c: Wrapping<u32> = digest[2];
	let mut d: Wrapping<u32> = digest[3];
	let mut e: Wrapping<u32> = digest[4];
	let mut f: Wrapping<u32> = digest[5];
	let mut g: Wrapping<u32> = digest[6];
	let mut h: Wrapping<u32> = digest[7];



	for t in 0..64 {
		t1 = h + shafuncs::ls1(e) + shafuncs::ch(e,f,g) + K[t] + w[t];
		t2 = shafuncs::ls0(a) + shafuncs::maj(a,b,c);
		h = g;
		g = f;
		f = e;
		e = d+t1;
		d = c;
		c = b;
		b = a;
		a = t1+t2;
	}


	digest[0] += a;
	digest[1] += b;
	digest[2] += c;
	digest[3] += d;
	digest[4] += e;
	digest[5] += f;
	digest[6] += g;
	digest[7] += h;

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


const K: [Wrapping<u32>; 64] = wraparray![

	0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
  	0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
	0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
	0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
	0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
	0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
	0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
	0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
	];
