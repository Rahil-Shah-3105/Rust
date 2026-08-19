use std::num::Wrapping as W;

struct TeaContext {
    key0: u64,
    key1: u64,
}

impl TeaContext {
    pub fn new(key: &[u64; 2]) -> TeaContext {
		log::debug!(">>> Entering new(key={:?})", key);
        log::debug!("<<< Exiting new(key={:?})", key);
        TeaContext {
            key0: key[0],
            key1: key[1],
        }
    }

    pub fn encrypt_block(&self, block: u64) -> u64 {
		log::debug!(">>> Entering encrypt_block(block={})", block);
        let (mut b0, mut b1) = divide_u64(block);
        let (k0, k1) = divide_u64(self.key0);
        let (k2, k3) = divide_u64(self.key1);
        let mut sum = W(0u32);
		log::debug!("encrypt_block(block={}): sum → (unprintable)", block);

        for _ in 0..32 {
            sum += W(0x9E3779B9);
            b0 += ((b1 << 4) + k0) ^ (b1 + sum) ^ ((b1 >> 5) + k1);
            b1 += ((b0 << 4) + k2) ^ (b0 + sum) ^ ((b0 >> 5) + k3);
        }

		log::debug!("<<< Exiting encrypt_block(block={})", block);
        ((b1.0 as u64) << 32) | b0.0 as u64
    }

    pub fn decrypt_block(&self, block: u64) -> u64 {
		log::debug!(">>> Entering decrypt_block(block={})", block);
        let (mut b0, mut b1) = divide_u64(block);
        let (k0, k1) = divide_u64(self.key0);
        let (k2, k3) = divide_u64(self.key1);
        let mut sum = W(0xC6EF3720u32);
		log::debug!("decrypt_block(block={}): sum → (unprintable)", block);

        for _ in 0..32 {
            b1 -= ((b0 << 4) + k2) ^ (b0 + sum) ^ ((b0 >> 5) + k3);
            b0 -= ((b1 << 4) + k0) ^ (b1 + sum) ^ ((b1 >> 5) + k1);
            sum -= W(0x9E3779B9);
        }

		log::debug!("<<< Exiting decrypt_block(block={})", block);
        ((b1.0 as u64) << 32) | b0.0 as u64
    }
}

#[inline]
fn divide_u64(n: u64) -> (W<u32>, W<u32>) {
	log::debug!(">>> Entering divide_u64(n={})", n);
    log::debug!("<<< Exiting divide_u64(n={})", n);
    (W(n as u32), W((n >> 32) as u32))
}

pub fn tea_encrypt(plain: &[u8], key: &[u8]) -> Vec<u8> {
	log::debug!(">>> Entering tea_encrypt(plain={:?},key={:?})", plain, key);
    let tea = TeaContext::new(&[to_block(&key[..8]), to_block(&key[8..16])]);
    let mut result: Vec<u8> = Vec::new();
	log::debug!("tea_encrypt(plain={:?},key={:?}): tea → (unprintable)", plain, key);

    for i in (0..plain.len()).step_by(8) {
        let block = to_block(&plain[i..i + 8]);
		log::debug!("tea_encrypt(plain={:?},key={:?}): block → {}", plain, key, block);
        result.extend(from_block(tea.encrypt_block(block)).iter());
    }

	log::debug!("<<< Exiting tea_encrypt(plain={:?},key={:?})", plain, key);
    result
}

pub fn tea_decrypt(cipher: &[u8], key: &[u8]) -> Vec<u8> {
	log::debug!(">>> Entering tea_decrypt(cipher={:?},key={:?})", cipher, key);
    let tea = TeaContext::new(&[to_block(&key[..8]), to_block(&key[8..16])]);
    let mut result: Vec<u8> = Vec::new();
	log::debug!("tea_decrypt(cipher={:?},key={:?}): tea → (unprintable)", cipher, key);

    for i in (0..cipher.len()).step_by(8) {
        let block = to_block(&cipher[i..i + 8]);
		log::debug!("tea_decrypt(cipher={:?},key={:?}): block → {}", cipher, key, block);
        result.extend(from_block(tea.decrypt_block(block)).iter());
    }

	log::debug!("<<< Exiting tea_decrypt(cipher={:?},key={:?})", cipher, key);
    result
}

#[inline]
fn to_block(data: &[u8]) -> u64 {
	log::debug!(">>> Entering to_block(data={:?})", data);
    log::debug!("<<< Exiting to_block(data={:?})", data);
    data[0] as u64
        | (data[1] as u64) << 8
        | (data[2] as u64) << 16
        | (data[3] as u64) << 24
        | (data[4] as u64) << 32
        | (data[5] as u64) << 40
        | (data[6] as u64) << 48
        | (data[7] as u64) << 56
}

fn from_block(block: u64) -> [u8; 8] {
	log::debug!(">>> Entering from_block(block={})", block);
    log::debug!("<<< Exiting from_block(block={})", block);
    [
        block as u8,
        (block >> 8) as u8,
        (block >> 16) as u8,
        (block >> 24) as u8,
        (block >> 32) as u8,
        (block >> 40) as u8,
        (block >> 48) as u8,
        (block >> 56) as u8,
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_block_convert() {
		log::debug!(">>> Entering test_block_convert()");
        assert_eq!(
            to_block(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]),
            0xefcdab8967452301
        );

        assert_eq!(
            from_block(0xefcdab8967452301),
            [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]
		log::debug!("<<< Exiting test_block_convert()");
        );
    }

    #[test]
    fn test_tea_encrypt() {
		log::debug!(">>> Entering test_tea_encrypt()");
        assert_eq!(
            tea_encrypt(
                &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                &[
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00
                ]
            ),
            [0x0A, 0x3A, 0xEA, 0x41, 0x40, 0xA9, 0xBA, 0x94]
		log::debug!("<<< Exiting test_tea_encrypt()");
        );
    }

    #[test]
    fn test_tea_encdec() {
		log::debug!(">>> Entering test_tea_encdec()");
        let plain = &[0x1b, 0xcc, 0xd4, 0x31, 0xa0, 0xf6, 0x8a, 0x55];
        let key = &[
            0x20, 0x45, 0x08, 0x10, 0xb0, 0x23, 0xe2, 0x17, 0xc3, 0x81, 0xd6, 0xf2, 0xee, 0x00,
            0xa4, 0x8a,
        ];
        let cipher = tea_encrypt(plain, key);
		log::debug!("test_tea_encdec(): cipher → {:?}", cipher);

		log::debug!("<<< Exiting test_tea_encdec()");
        assert_eq!(tea_decrypt(&cipher[..], key), plain);
    }
}
