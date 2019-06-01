const WORDSIZE_BITS: u32 = 32;
const ZEROPAD: u32 = 488;
const CHUNKBITS: u32 = 512;
const MLEN_BYTES: u32 = 8;


fn rotr(x: u32, n: u32) -> u32 {
	(x >> n) | (x << (WORDSIZE_BITS - n) )
}

fn ch(x: u32, y: u32, z: u32) -> u32 {
	(x & y) ^ ((!x) & z)
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
	(x & y) ^ (x & z) ^ (y & z)
}

fn S0(x: u32) -> u32 {
	rotr(x, 2) ^ rotr(x,13) ^ rotr(x,22)
}

fn S1(x: u32) -> u32 {
	rotr(x,6) ^ rotr(x,11) ^ rotr(x,22)
}

fn s0(x: u32) -> u32 {
	rotr(x,7) ^ rotr(x,18) ^ (x >> 3)
}

fn s1(x: u32) -> u32 {
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

	println!("Numzeros: {}",numzeros);

}