
use std::num::Wrapping;
const WORDSIZE_BITS: usize = 32;
const ZEROPAD: u32 = 488;
const CHUNKBITS: u32 = 512;
const MLEN_BYTES: u32 = 8;


fn rotr(x: Wrapping<u32>, n: usize) -> Wrapping<u32> {
	(x >> n) | (x << (WORDSIZE_BITS - n) )
}

pub fn ch(x: Wrapping<u32>, y: Wrapping<u32>, z: Wrapping<u32>) -> Wrapping<u32> {
	(x & y) ^ ((!x) & z)
}

pub fn maj(x: Wrapping<u32>, y: Wrapping<u32>, z: Wrapping<u32>) -> Wrapping<u32> {
	(x & y) ^ (x & z) ^ (y & z)
}

pub fn S0(x: Wrapping<u32>) -> Wrapping<u32> {
	rotr(x, 2) ^ rotr(x,13) ^ rotr(x,22)
}

pub fn S1(x: Wrapping<u32>) -> Wrapping<u32> {
	rotr(x,6) ^ rotr(x,11) ^ rotr(x,22)
}

fn s0(x: Wrapping<u32>) -> Wrapping<u32> {
	rotr(x,7) ^ rotr(x,18) ^ (x >> 3)
}

fn s1(x: Wrapping<u32>) -> Wrapping<u32> {
	rotr(x,17) ^ rotr(x,19) ^ (x >> 10)
}

pub fn padmessage(bytebuffer : &mut [u8], readbyte : usize, messagelength: u64) -> () {
	let bitsize: u64 = messagelength * 8;
	let mut pos: usize = readbyte ;
	bytebuffer[pos] = 0x80;
	let mut numzeros: u8 = 0;
	pos+=1;
	for i in pos..(64-8) {
		bytebuffer[i] = 0;
		numzeros+=1;
	}
	
	println!("{:064b}", bitsize);
	let mut v: u8;
	for i in 0..8 {
		v = ((bitsize >> i*8) & 0xff) as u8;
		bytebuffer[63-i] = v;
		println!("{:08b}", v);
	}

}

pub fn message_schedule(wordbuffer: [Wrapping<u32>; 16]) -> [Wrapping<u32>; 64] {
	let mut ms: [Wrapping<u32>; 64] = [Wrapping(0); 64];
	ms[..16].clone_from_slice(&wordbuffer);
	println!("{:?}", wordbuffer);
	for i in ms.iter() {
		println!("{}", i);
	}

	ms
}