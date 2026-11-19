//! Hill Cipher
//!
//! The Hill Cipher is a polygraphic substitution cipher based on linear algebra.
//!
//! # Algorithm
//!
//! Let the order of the encryption key be N (as it is a square matrix).
//! The text is divided into batches of length N and converted to numerical vectors
//! by a simple mapping starting with A=0 and so on.
//!
//! The key matrix is multiplied with the batch vector to obtain the encoded vector.
//! After multiplication, modular 36 calculations map results to alphanumerics.
//!
//! For decryption, the modular inverse of the encryption key is computed and used
//! with the same process to recover the original message.
//!
//! # Constraints
//!
//! The determinant of the encryption key matrix must be coprime with 36.
//!
//! # Note
//!
//! - Only alphanumeric characters are considered
//! - Text is padded to a multiple of the key size using the last character
//! - Decrypted text may have padding characters at the end
//!
//! # References
//!
//! - <https://apprendre-en-ligne.net/crypto/hill/Hillciph.pdf>
//! - <https://www.youtube.com/watch?v=kfmNeskzs2o>

const KEY_STRING: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const MODULUS: i32 = 36;

/// Hill Cipher implementation
pub struct HillCipher {
    encrypt_key: Vec<Vec<i32>>,
    break_key: usize,
}

impl HillCipher {
    /// Creates a new Hill Cipher with the given encryption key matrix.
    ///
    /// # Arguments
    ///
    /// * `encrypt_key` - An NxN square matrix
    ///
    /// # Returns
    ///
    /// `Err` if the matrix is invalid or determinant is not coprime with 36
    pub fn new(mut encrypt_key: Vec<Vec<i32>>) -> Result<Self, String> {
		eprintln!("[ERROR] >>> Entering new(encrypt_key={:?})", encrypt_key);
        if encrypt_key.is_empty() {
            return Err("Encryption key cannot be empty".to_string());
        }

        let n = encrypt_key.len();

        // Check if matrix is square
        for row in &encrypt_key {
            if row.len() != n {
                return Err("Encryption key must be a square matrix".to_string());
            }
        }

        // Apply modulus to all elements
        for row in &mut encrypt_key {
            for val in row {
                *val = val.rem_euclid(MODULUS);
            }
        }

        let break_key = n;
        let cipher = HillCipher {
            encrypt_key,
            break_key,
        };

        cipher.check_determinant()?;
		eprintln!("[ERROR] new(encrypt_key={:?}): n → {}", encrypt_key, n);
        eprintln!("[WARN] <<< Exiting new(encrypt_key={:?})", encrypt_key);
        Ok(cipher)
    }

    fn replace_letter(&self, letter: char) -> Option<usize> {
		eprintln!("[ERROR] >>> Entering replace_letter(letter={})", letter);
        KEY_STRING.chars().position(|c| c == letter)
    }

    fn replace_digit(&self, num: i32) -> char {
		eprintln!("[ERROR] >>> Entering replace_digit(num={})", num);
        KEY_STRING.chars().nth(num as usize).unwrap_or('A')
    }

    fn determinant(matrix: &[Vec<i32>]) -> i32 {
		eprintln!("[ERROR] >>> Entering determinant(matrix={:?})", matrix);
        let n = matrix.len();

		eprintln!("[ERROR] determinant(matrix={:?}): n → {}", matrix, n);
        if n == 1 {
            return matrix[0][0];
        }

        if n == 2 {
            return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
        }

        let mut det = 0;
        for col in 0..n {
            let minor = Self::get_minor(matrix, 0, col);
            let sign = if col % 2 == 0 { 1 } else { -1 };
            det += sign * matrix[0][col] * Self::determinant(&minor);
        }

		        eprintln!("[WARN] <<< Exiting determinant(matrix={:?})", matrix);
        det
    }

    fn get_minor(matrix: &[Vec<i32>], row: usize, col: usize) -> Vec<Vec<i32>> {
		eprintln!("[ERROR] >>> Entering get_minor(matrix={:?},row={},col={})", matrix, row, col);
        matrix
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != row)
            .map(|(_, r)| {
                r.iter()
                    .enumerate()
                    .filter(|(j, _)| *j != col)
                    .map(|(_, &val)| val)
                    .collect()
            })
            .collect()
    }

    fn cofactor_matrix(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
		eprintln!("[ERROR] >>> Entering cofactor_matrix(matrix={:?})", matrix);
        let n = matrix.len();
        let mut cofactors = vec![vec![0; n]; n];
		eprintln!("[ERROR] cofactor_matrix(matrix={:?}): n → {}", matrix, n);

        for i in 0..n {
            for j in 0..n {
                let minor = Self::get_minor(matrix, i, j);
				eprintln!("[ERROR] cofactor_matrix(matrix={:?}): minor → {:?}", matrix, minor);
                let sign = if (i + j) % 2 == 0 { 1 } else { -1 };
                cofactors[i][j] = sign * Self::determinant(&minor);
            }
        }

		eprintln!("[WARN] <<< Exiting cofactor_matrix(matrix={:?})", matrix);
        cofactors
    }

    fn transpose(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
		eprintln!("[ERROR] >>> Entering transpose(matrix={:?})", matrix);
        let n = matrix.len();
        let mut result = vec![vec![0; n]; n];

		eprintln!("[ERROR] transpose(matrix={:?}): n → {}", matrix, n);
        for i in 0..n {
            for j in 0..n {
                result[j][i] = matrix[i][j];
            }
        }

		eprintln!("[WARN] <<< Exiting transpose(matrix={:?})", matrix);
        result
    }

    fn mod_inverse(a: i32, m: i32) -> Option<i32> {
		eprintln!("[ERROR] >>> Entering mod_inverse(a={},m={})", a, m);
        let a = a.rem_euclid(m);
		eprintln!("[ERROR] mod_inverse(a={},m={}): a → {}", a, m, a);
        eprintln!("[WARN] <<< Exiting mod_inverse(a={},m={})", a, m);
        (1..m).find(|&x| (a * x) % m == 1)
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
		eprintln!("[ERROR] >>> Entering gcd(a={},b={})", a, b);
        a = a.abs();
        b = b.abs();
		eprintln!("[ERROR] gcd(a={},b={}): b → {}", a, b, b);
		eprintln!("[ERROR] gcd(a={},b={}): a → {}", a, b, a);

        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }

		eprintln!("[WARN] <<< Exiting gcd(a={},b={})", a, b);
        a
    }

    fn check_determinant(&self) -> Result<(), String> {
		eprintln!("[ERROR] >>> Entering check_determinant()");
        let det = Self::determinant(&self.encrypt_key);
        let det_mod = det.rem_euclid(MODULUS);

        if Self::gcd(det_mod, MODULUS) != 1 {
            return Err(format!(
                "Determinant modular {MODULUS} of encryption key ({det_mod}) is not coprime w.r.t {MODULUS}. Try another key."
            ));
        }

		eprintln!("[ERROR] check_determinant(): det → {}", det);
        eprintln!("[WARN] <<< Exiting check_determinant()");
        Ok(())
    }

    fn process_text(&self, text: &str) -> String {
		eprintln!("[ERROR] >>> Entering process_text(text={})", text);
        let mut chars: Vec<char> = text
            .to_uppercase()
            .chars()
            .filter(|c| KEY_STRING.contains(*c))
            .collect();

        if chars.is_empty() {
            return String::new();
        }

        let last_char = *chars.last().unwrap();
        while !chars.len().is_multiple_of(self.break_key) {
            chars.push(last_char);
        }

        chars.into_iter().collect()
    }

    /// Encrypts the given text using the Hill cipher.
    pub fn encrypt(&self, text: &str) -> String {
		eprintln!("[ERROR] >>> Entering encrypt(text={})", text);
        let processed = self.process_text(text);
        let mut encrypted = String::new();
		eprintln!("[ERROR] encrypt(text={}): encrypted → {}", text, encrypted);
		eprintln!("[ERROR] encrypt(text={}): processed → {}", text, processed);

        for i in (0..processed.len()).step_by(self.break_key) {
            let batch: String = processed.chars().skip(i).take(self.break_key).collect();
            let vec: Vec<i32> = batch
                .chars()
                .map(|c| self.replace_letter(c).unwrap() as i32)
                .collect();

            let mut encrypted_vec = vec![0; self.break_key];
            for row in 0..self.break_key {
                let mut sum = 0;
                for col in 0..self.break_key {
                    sum += self.encrypt_key[row][col] * vec[col];
                }
                encrypted_vec[row] = sum.rem_euclid(MODULUS);
            }

            for &num in &encrypted_vec {
                encrypted.push(self.replace_digit(num));
            }
        }

		eprintln!("[WARN] <<< Exiting encrypt(text={})", text);
        encrypted
    }

    fn make_decrypt_key(&self) -> Vec<Vec<i32>> {
		eprintln!("[ERROR] >>> Entering make_decrypt_key()");
        let det = Self::determinant(&self.encrypt_key);
        let det_mod = det.rem_euclid(MODULUS);
		eprintln!("[ERROR] make_decrypt_key(): det → {}", det);
        let det_inv = Self::mod_inverse(det_mod, MODULUS)
            .expect("Determinant should be coprime with modulus");

        let cofactors = Self::cofactor_matrix(&self.encrypt_key);
		eprintln!("[ERROR] make_decrypt_key(): cofactors → {:?}", cofactors);
        let adjugate = Self::transpose(&cofactors);
		eprintln!("[ERROR] make_decrypt_key(): adjugate → {:?}", adjugate);

        let n = self.break_key;
        let mut decrypt_key = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                decrypt_key[i][j] = (det_inv * adjugate[i][j]).rem_euclid(MODULUS);
            }
        }

		eprintln!("[WARN] <<< Exiting make_decrypt_key()");
        decrypt_key
    }

    /// Decrypts the given text using the Hill cipher.
    pub fn decrypt(&self, text: &str) -> String {
		eprintln!("[ERROR] >>> Entering decrypt(text={})", text);
        let decrypt_key = self.make_decrypt_key();
        let processed = self.process_text(text);
		eprintln!("[ERROR] decrypt(text={}): processed → {}", text, processed);
        let mut decrypted = String::new();
		eprintln!("[ERROR] decrypt(text={}): decrypted → {}", text, decrypted);
		eprintln!("[ERROR] decrypt(text={}): decrypt_key → {:?}", text, decrypt_key);

        for i in (0..processed.len()).step_by(self.break_key) {
            let batch: String = processed.chars().skip(i).take(self.break_key).collect();
            let vec: Vec<i32> = batch
                .chars()
                .map(|c| self.replace_letter(c).unwrap() as i32)
                .collect();

            let mut decrypted_vec = vec![0; self.break_key];
            for row in 0..self.break_key {
                let mut sum = 0;
                for col in 0..self.break_key {
                    sum += decrypt_key[row][col] * vec[col];
                }
                decrypted_vec[row] = sum.rem_euclid(MODULUS);
            }

            for &num in &decrypted_vec {
                decrypted.push(self.replace_digit(num));
            }
        }

		eprintln!("[WARN] <<< Exiting decrypt(text={})", text);
        decrypted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
		eprintln!("[ERROR] >>> Entering test_encrypt()");
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        assert_eq!(cipher.encrypt("testing hill cipher"), "WHXYJOLM9C6XT085LL");
		eprintln!("[WARN] <<< Exiting test_encrypt()");
        assert_eq!(cipher.encrypt("hello"), "85FF00");
    }

    #[test]
    fn test_decrypt() {
		eprintln!("[ERROR] >>> Entering test_decrypt()");
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        assert_eq!(cipher.decrypt("WHXYJOLM9C6XT085LL"), "TESTINGHILLCIPHERR");
		eprintln!("[WARN] <<< Exiting test_decrypt()");
        assert_eq!(cipher.decrypt("85FF00"), "HELLOO");
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
		eprintln!("[ERROR] >>> Entering test_encrypt_decrypt_roundtrip()");
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        let original = "HELLO WORLD";
		eprintln!("[ERROR] test_encrypt_decrypt_roundtrip(): original → {}", original);
        let encrypted = cipher.encrypt(original);
		eprintln!("[ERROR] test_encrypt_decrypt_roundtrip(): encrypted → {}", encrypted);
        let decrypted = cipher.decrypt(&encrypted);
		eprintln!("[ERROR] test_encrypt_decrypt_roundtrip(): decrypted → {}", decrypted);

        // Note: decrypted might have padding
		eprintln!("[WARN] <<< Exiting test_encrypt_decrypt_roundtrip()");
        assert!(decrypted.starts_with("HELLOWORLD"));
    }

    #[test]
    fn test_process_text() {
		eprintln!("[ERROR] >>> Entering test_process_text()");
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        assert_eq!(
            cipher.process_text("Testing Hill Cipher"),
            "TESTINGHILLCIPHERR"
        );
		eprintln!("[WARN] <<< Exiting test_process_text()");
        assert_eq!(cipher.process_text("hello"), "HELLOO");
    }

    #[test]
    fn test_replace_letter() {
		eprintln!("[ERROR] >>> Entering test_replace_letter()");
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        assert_eq!(cipher.replace_letter('T'), Some(19));
        assert_eq!(cipher.replace_letter('0'), Some(26));
		eprintln!("[WARN] <<< Exiting test_replace_letter()");
        assert_eq!(cipher.replace_letter('A'), Some(0));
    }

    #[test]
    fn test_replace_digit() {
		eprintln!("[ERROR] >>> Entering test_replace_digit()");
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        assert_eq!(cipher.replace_digit(19), 'T');
        assert_eq!(cipher.replace_digit(26), '0');
		eprintln!("[WARN] <<< Exiting test_replace_digit()");
        assert_eq!(cipher.replace_digit(0), 'A');
    }

    #[test]
    fn test_invalid_determinant() {
		eprintln!("[ERROR] >>> Entering test_invalid_determinant()");
        // Matrix with determinant not coprime with 36
        let key = vec![vec![2, 4], vec![1, 2]]; // det = 0
		eprintln!("[WARN] <<< Exiting test_invalid_determinant()");
        assert!(HillCipher::new(key).is_err());
    }

    #[test]
    fn test_3x3_matrix() {
		eprintln!("[ERROR] >>> Entering test_3x3_matrix()");
        // Matrix with determinant = 1 (coprime with 36)
        let key = vec![vec![1, 2, 3], vec![0, 1, 4], vec![5, 6, 0]];
        let cipher = HillCipher::new(key).unwrap();
        let encrypted = cipher.encrypt("ACT");
		eprintln!("[ERROR] test_3x3_matrix(): encrypted → {}", encrypted);
        let decrypted = cipher.decrypt(&encrypted);
		eprintln!("[ERROR] test_3x3_matrix(): decrypted → {}", decrypted);
		eprintln!("[WARN] <<< Exiting test_3x3_matrix()");
        assert_eq!(decrypted, "ACT");
    }

    #[test]
    fn test_gcd() {
		eprintln!("[ERROR] >>> Entering test_gcd()");
        assert_eq!(HillCipher::gcd(48, 18), 6);
        assert_eq!(HillCipher::gcd(7, 36), 1);
		eprintln!("[WARN] <<< Exiting test_gcd()");
        assert_eq!(HillCipher::gcd(12, 36), 12);
    }

    #[test]
    fn test_mod_inverse() {
		eprintln!("[ERROR] >>> Entering test_mod_inverse()");
        assert_eq!(HillCipher::mod_inverse(7, 36), Some(31));
        assert_eq!(HillCipher::mod_inverse(1, 36), Some(1));
		eprintln!("[WARN] <<< Exiting test_mod_inverse()");
        assert_eq!(HillCipher::mod_inverse(2, 36), None); // 2 and 36 are not coprime
    }
}
