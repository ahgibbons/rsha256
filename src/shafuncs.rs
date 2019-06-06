
use std::num::Wrapping;
const WORDSIZE_BITS: usize = 32;
const LENBYTES: usize = 8;
pub const CHUNKBYTES: usize = 64;
const ONEPAD: u8 = 0x80;

fn rotr(x: Wrapping<u32>, n: usize) -> Wrapping<u32> {
	(x >> n) | (x << (WORDSIZE_BITS - n) )
}

pub fn ch(x: Wrapping<u32>, y: Wrapping<u32>, z: Wrapping<u32>) -> Wrapping<u32> {
	(x & y) ^ ((!x) & z)
}

pub fn maj(x: Wrapping<u32>, y: Wrapping<u32>, z: Wrapping<u32>) -> Wrapping<u32> {
	(x & y) ^ (x & z) ^ (y & z)
}

pub fn ls0(x: Wrapping<u32>) -> Wrapping<u32> {
	rotr(x, 2) ^ rotr(x,13) ^ rotr(x,22)
}

pub fn ls1(x: Wrapping<u32>) -> Wrapping<u32> {
	rotr(x,6) ^ rotr(x,11) ^ rotr(x,25)
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
	bytebuffer[pos] = ONEPAD;
	pos+=1;
	for i in pos..(CHUNKBYTES-LENBYTES) {
		bytebuffer[i] = 0;
	}

	let mut v: u8;
	for i in 0..8 {
		v = ((bitsize >> i*8) & 0xff) as u8;
		bytebuffer[63-i] = v;
	}

}

pub fn message_schedule(wordbuffer: [Wrapping<u32>; 16]) -> [Wrapping<u32>; 64] {
	let mut ms: [Wrapping<u32>; 64] = [Wrapping(0); 64];
	ms[..16].clone_from_slice(&wordbuffer);
	for t in 16..64 {
		ms[t] = s1(ms[t-2]) + ms[t-7] + s0(ms[t-15]) + ms[t-16];
	}

	ms
}