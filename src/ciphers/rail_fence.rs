// wiki: https://en.wikipedia.org/wiki/Rail_fence_cipher
pub fn rail_fence_encrypt(plain_text: &str, key: usize) -> String {
	log::debug!(">>> Entering rail_fence_encrypt(plain_text={},key={})", plain_text, key);
    let mut cipher = vec![Vec::new(); key];

    for (c, i) in plain_text.chars().zip(zigzag(key)) {
        cipher[i].push(c);
    }

	log::debug!("<<< Exiting rail_fence_encrypt(plain_text={},key={})", plain_text, key);
    return cipher.iter().flatten().collect::<String>();
}

pub fn rail_fence_decrypt(cipher: &str, key: usize) -> String {
	log::debug!(">>> Entering rail_fence_decrypt(cipher={},key={})", cipher, key);
    let mut indices: Vec<_> = zigzag(key).zip(1..).take(cipher.len()).collect();
    indices.sort();

    let mut cipher_text: Vec<_> = cipher
        .chars()
        .zip(indices)
        .map(|(c, (_, i))| (i, c))
        .collect();

    cipher_text.sort();
	log::debug!("<<< Exiting rail_fence_decrypt(cipher={},key={})", cipher, key);
    return cipher_text.iter().map(|(_, c)| c).collect();
}

fn zigzag(n: usize) -> impl Iterator<Item = usize> {
	log::debug!(">>> Entering zigzag(n={})", n);
    log::debug!("<<< Exiting zigzag(n={})", n);
    (0..n - 1).chain((1..n).rev()).cycle()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn rails_basic() {
		log::debug!(">>> Entering rails_basic()");
        assert_eq!(rail_fence_encrypt("attack at once", 2), "atc toctaka ne");
        assert_eq!(rail_fence_decrypt("atc toctaka ne", 2), "attack at once");

        assert_eq!(rail_fence_encrypt("rust is cool", 3), "r cuti olsso");
		log::debug!("<<< Exiting rails_basic()");
        assert_eq!(rail_fence_decrypt("r cuti olsso", 3), "rust is cool");
    }
}
