//! Huffman Encoding implementation
//!
//! Huffman coding is a lossless data compression algorithm that assigns variable-length codes
//! to characters based on their frequency of occurrence. Characters that occur more frequently
//! are assigned shorter codes, while less frequent characters get longer codes.
//!
//! # Algorithm Overview
//!
//! 1. Count the frequency of each character in the input
//! 2. Build a min-heap (priority queue) of nodes based on frequency
//! 3. Build the Huffman tree by repeatedly:
//!    - Remove two nodes with minimum frequency
//!    - Create a parent node with combined frequency
//!    - Insert the parent back into the heap
//! 4. Traverse the tree to assign binary codes to each character
//! 5. Encode the input using the generated codes
//!
//! # Time Complexity
//!
//! - Building frequency map: O(n) where n is input length
//! - Building Huffman tree: O(m log m) where m is number of unique characters
//! - Encoding: O(n)
//!
//! # Usage
//!
//! As a library:
//! ```no_run
//! use the_algorithms_rust::compression::huffman_encode;
//!
//! let text = "hello world";
//! let (encoded, codes) = huffman_encode(text);
//! println!("Original: {}", text);
//! println!("Encoded: {}", encoded);
//! ```
//!
//! As a command-line tool:
//! ```bash
//! rustc huffman_encoding.rs -o huffman
//! ./huffman input.txt
//! ```

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[cfg(not(test))]
use std::env;

/// Represents a node in the Huffman tree
#[derive(Debug, Eq, PartialEq)]
enum HuffmanNode {
    /// Leaf node containing a character and its frequency
    Leaf { character: char, frequency: usize },
    /// Internal node with combined frequency and left/right children
    Internal {
        frequency: usize,
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
    },
}

impl HuffmanNode {
    /// Returns the frequency of this node
    fn frequency(&self) -> usize {
		log::debug!(">>> Entering frequency()");
        match self {
            HuffmanNode::Leaf { frequency, .. } | HuffmanNode::Internal { frequency, .. } => {
				log::debug!("<<< Exiting frequency()");
                *frequency
            }
        }
    }

    /// Creates a new leaf node
    fn new_leaf(character: char, frequency: usize) -> Self {
		log::debug!(">>> Entering new_leaf(character={},frequency={})", character, frequency);
        log::debug!("<<< Exiting new_leaf(character={},frequency={})", character, frequency);
        HuffmanNode::Leaf {
            character,
            frequency,
        }
    }

    /// Creates a new internal node from two children
    fn new_internal(left: HuffmanNode, right: HuffmanNode) -> Self {
		log::debug!(">>> Entering new_internal(left={:?},right={:?})", left, right);
        let frequency = left.frequency() + right.frequency();
		log::debug!("new_internal(left={:?},right={:?}): frequency → {}", left, right, frequency);
        log::debug!("<<< Exiting new_internal(left={:?},right={:?})", left, right);
        HuffmanNode::Internal {
            frequency,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// Wrapper for HuffmanNode to implement Ord for BinaryHeap (min-heap)
#[derive(Eq, PartialEq)]
struct HeapNode(HuffmanNode);

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
		log::debug!(">>> Entering cmp(other=(unprintable))");
        // Reverse ordering for min-heap
		log::debug!("<<< Exiting cmp(other=(unprintable))");
        other.0.frequency().cmp(&self.0.frequency())
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		log::debug!(">>> Entering partial_cmp(other=(unprintable))");
        log::debug!("<<< Exiting partial_cmp(other=(unprintable))");
        Some(self.cmp(other))
    }
}

/// Counts the frequency of each character in the input string
///
/// # Arguments
///
/// * `text` - The input string to analyze
///
/// # Returns
///
/// A HashMap mapping each character to its frequency count
fn build_frequency_map(text: &str) -> HashMap<char, usize> {
	log::debug!(">>> Entering build_frequency_map(text={})", text);
    let mut frequencies = HashMap::new();
    for ch in text.chars() {
		log::debug!("build_frequency_map(text={}): frequencies → (unprintable)", text);
        *frequencies.entry(ch).or_insert(0) += 1;
    }
	log::debug!("<<< Exiting build_frequency_map(text={})", text);
    frequencies
}

/// Builds the Huffman tree from a frequency map
///
/// # Arguments
///
/// * `frequencies` - HashMap of character frequencies
///
/// # Returns
///
/// The root node of the Huffman tree, or None if input is empty
fn build_huffman_tree(frequencies: HashMap<char, usize>) -> Option<HuffmanNode> {
	log::debug!(">>> Entering build_huffman_tree(frequencies={:?})", frequencies);
    if frequencies.is_empty() {
        return None;
    }

    let mut heap: BinaryHeap<HeapNode> = frequencies
        .into_iter()
        .map(|(ch, freq)| HeapNode(HuffmanNode::new_leaf(ch, freq)))
        .collect();

    // Special case: only one unique character
    if heap.len() == 1 {
        return heap.pop().map(|node| node.0);
    }

    // Build the tree by combining nodes
    while heap.len() > 1 {
        let left = heap.pop().unwrap().0;
		log::debug!("build_huffman_tree(frequencies={:?}): left → (unprintable)", frequencies);
		log::debug!("build_huffman_tree(frequencies={:?}): right → (unprintable)", frequencies);
        let right = heap.pop().unwrap().0;
        let parent = HuffmanNode::new_internal(left, right);
        heap.push(HeapNode(parent));
    }

	log::debug!("build_huffman_tree(frequencies={:?}): parent → (unprintable)", frequencies);
    log::debug!("<<< Exiting build_huffman_tree(frequencies={:?})", frequencies);
    heap.pop().map(|node| node.0)
}

/// Traverses the Huffman tree to generate binary codes for each character
///
/// # Arguments
///
/// * `node` - The current node being traversed
/// * `code` - The current binary code string
/// * `codes` - HashMap to store the generated codes
fn generate_codes(node: &HuffmanNode, code: String, codes: &mut HashMap<char, String>) {
	log::debug!(">>> Entering generate_codes(node={:?},code={},codes={:?})", node, code, codes);
    match node {
        HuffmanNode::Leaf { character, .. } => {
            // Use "0" for single character case
            codes.insert(
                *character,
                if code.is_empty() {
                    "0".to_string()
                } else {
                    code
                },
            );
        }
        HuffmanNode::Internal { left, right, .. } => {
            generate_codes(left, format!("{code}0"), codes);
            generate_codes(right, format!("{code}1"), codes);
        }
	log::debug!("<<< Exiting generate_codes(node={:?},code={},codes={:?})", node, code, codes);
    }
}

/// Encodes text using Huffman coding
///
/// # Arguments
///
/// * `text` - The input string to encode
///
/// # Returns
///
/// A tuple containing:
/// - The encoded binary string
/// - A HashMap of character to binary code mappings
///
/// # Examples
///
/// ```
/// # use std::collections::HashMap;
/// # use the_algorithms_rust::compression::huffman_encode;
/// let (encoded, codes) = huffman_encode("hello");
/// assert!(!encoded.is_empty());
/// assert!(codes.contains_key(&'h'));
/// ```
pub fn huffman_encode(text: &str) -> (String, HashMap<char, String>) {
	log::debug!(">>> Entering huffman_encode(text={})", text);
    if text.is_empty() {
		log::debug!("huffman_encode(text={}): codes → (unprintable)", text);
    log::debug!("<<< Exiting huffman_encode(text={})", text);
        return (String::new(), HashMap::new());
    }

    let frequencies = build_frequency_map(text);
    let tree = build_huffman_tree(frequencies).expect("Failed to build Huffman tree");

    let mut codes = HashMap::new();
    generate_codes(&tree, String::new(), &mut codes);

    let encoded: String = text.chars().map(|ch| codes[&ch].as_str()).collect();

    (encoded, codes)
}

/// Decodes a Huffman-encoded string
///
/// # Arguments
///
/// * `encoded` - The binary string to decode
/// * `codes` - HashMap of character to binary code mappings
///
/// # Returns
///
/// The decoded original string
///
/// # Examples
///
/// ```
/// # use std::collections::HashMap;
/// # use the_algorithms_rust::compression::{huffman_encode, huffman_decode};
/// let text = "hello world";
/// let (encoded, codes) = huffman_encode(text);
/// let decoded = huffman_decode(&encoded, &codes);
/// assert_eq!(text, decoded);
/// ```
pub fn huffman_decode(encoded: &str, codes: &HashMap<char, String>) -> String {
	log::debug!(">>> Entering huffman_decode(encoded={},codes={:?})", encoded, codes);
    if encoded.is_empty() {
        return String::new();
    }

    // Reverse the code map for decoding
    let reverse_codes: HashMap<&str, char> = codes
        .iter()
        .map(|(ch, code)| (code.as_str(), *ch))
        .collect();

    let mut decoded = String::new();
	log::debug!("huffman_decode(encoded={},codes={:?}): decoded → {}", encoded, codes, decoded);
    let mut current_code = String::new();

    for bit in encoded.chars() {
        current_code.push(bit);
        if let Some(&character) = reverse_codes.get(current_code.as_str()) {
            decoded.push(character);
            current_code.clear();
        }
    }

    decoded
}

/// Demonstrates Huffman encoding by processing a file and displaying detailed results
///
/// This function reads a file, encodes it using Huffman coding, and displays:
/// - Character code mappings
/// - Compression statistics
/// - Encoded output (with smart truncation for large files)
/// - Decoding verification
///
/// # Arguments
///
/// * `file_path` - Path to the file to encode
///
/// # Returns
///
/// Result indicating success or IO error
///
/// # Examples
///
/// ```ignore
/// // Note: This function is not re-exported in the public API
/// // Access it via: the_algorithms_rust::compression::huffman_encoding::demonstrate_huffman_from_file
/// use std::fs::File;
/// use std::io::Write;
///
/// // Create a test file
/// let mut file = File::create("test.txt").unwrap();
/// file.write_all(b"hello world").unwrap();
///
/// // Demonstrate Huffman encoding
/// // In your code, use the full path or import from huffman_encoding module
/// demonstrate_huffman_from_file("test.txt").unwrap();
/// ```
#[allow(dead_code)]
pub fn demonstrate_huffman_from_file(file_path: &str) -> std::io::Result<()> {
	log::debug!(">>> Entering demonstrate_huffman_from_file(file_path={})", file_path);
    // Read the file contents
    let text = fs::read_to_string(file_path)?;
	log::debug!("demonstrate_huffman_from_file(file_path={}): text → (unprintable)", file_path);

    if text.is_empty() {
        println!("File is empty!");
        return Ok(());
    }

    // Encode using Huffman coding
    let (encoded, codes) = huffman_encode(&text);

    // Display the results
    println!("Huffman Coding of {file_path}: ");
    println!();

    // Show the code table
    println!("Character Codes:");
    println!("{:-<40}", "");
    let mut sorted_codes: Vec<_> = codes.iter().collect();
    sorted_codes.sort_by_key(|(ch, _)| *ch);

    for (ch, code) in sorted_codes {
        let display_char = if ch.is_whitespace() {
            format!("'{}' (space/whitespace)", ch.escape_default())
        } else {
            format!("'{ch}'")
        };
        println!("{display_char:20} -> {code}");
    }
    println!("{:-<40}", "");
    println!();

    // Show encoding statistics
    let original_bits = text.len() * 8; // Assuming 8-bit characters
	log::debug!("demonstrate_huffman_from_file(file_path={}): original_bits → (unprintable)", file_path);
    let compressed_bits = encoded.len();
	log::debug!("demonstrate_huffman_from_file(file_path={}): compressed_bits → {}", file_path, compressed_bits);
    let compression_ratio = if original_bits > 0 {
        (1.0 - (compressed_bits as f64 / original_bits as f64)) * 100.0
    } else {
        0.0
    };

    println!("Statistics:");
    println!(
        "  Original size:    {} characters ({} bits)",
        text.len(),
        original_bits
    );
    println!("  Encoded size:     {compressed_bits} bits");
    println!("  Compression:      {compression_ratio:.2}%");
    println!();

    // Show the encoded output (limited to avoid overwhelming the terminal)
    println!("Encoded output:");
    if encoded.len() <= 500 {
        // Split into chunks of 50 for readability
        for (i, chunk) in encoded.as_bytes().chunks(50).enumerate() {
            print!("{:4}: ", i * 50);
            for &byte in chunk {
                print!("{}", byte as char);
            }
            println!();
        }
    } else {
        // Show first and last portions for very long outputs
        println!("  (showing first and last 200 bits)");
        print!("  Start: ");
        for &byte in &encoded.as_bytes()[..200] {
            print!("{}", byte as char);
        }
        println!();
        print!("  End:   ");
        for &byte in &encoded.as_bytes()[encoded.len() - 200..] {
            print!("{}", byte as char);
        }
        println!();
    }
    println!();

    // Verify decoding
    let decoded = huffman_decode(&encoded, &codes);
	log::debug!("demonstrate_huffman_from_file(file_path={}): decoded → {}", file_path, decoded);
    if decoded == text {
        println!("✓ Decoding verification: SUCCESS");
    } else {
        println!("✗ Decoding verification: FAILED");
    }

    log::debug!("<<< Exiting demonstrate_huffman_from_file(file_path={})", file_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
		log::debug!(">>> Entering test_empty_string()");
        let (encoded, codes) = huffman_encode("");
        assert_eq!(encoded, "");
		log::debug!("<<< Exiting test_empty_string()");
        assert!(codes.is_empty());
    }

    #[test]
    fn test_single_character() {
		log::debug!(">>> Entering test_single_character()");
        let (encoded, codes) = huffman_encode("aaaa");
        assert_eq!(encoded, "0000");
		log::debug!("<<< Exiting test_single_character()");
        assert_eq!(codes.get(&'a'), Some(&"0".to_string()));
    }

    #[test]
    fn test_simple_string() {
		log::debug!(">>> Entering test_simple_string()");
        let text = "hello";
        let (encoded, codes) = huffman_encode(text);
		log::debug!("test_simple_string(): text → {}", text);

        // Verify all characters have codes
        for ch in text.chars() {
            assert!(codes.contains_key(&ch), "Missing code for '{ch}'");
        }

        // Verify decoding returns original text
        let decoded = huffman_decode(&encoded, &codes);
		log::debug!("test_simple_string(): decoded → {}", decoded);
		log::debug!("<<< Exiting test_simple_string()");
        assert_eq!(decoded, text);
    }

    #[test]
    fn test_encode_decode_roundtrip() {
		log::debug!(">>> Entering test_encode_decode_roundtrip()");
        let test_cases = vec![
            "a",
            "ab",
            "hello world",
            "the quick brown fox jumps over the lazy dog",
            "aaaaabbbbbcccccdddddeeeeefffffggggghhhhhiiiii",
        ];

        for text in test_cases {
            let (encoded, codes) = huffman_encode(text);
            let decoded = huffman_decode(&encoded, &codes);
			log::debug!("test_encode_decode_roundtrip(): decoded → {}", decoded);
            assert_eq!(decoded, text, "Failed roundtrip for: '{text}'");
		log::debug!("<<< Exiting test_encode_decode_roundtrip()");
        }
    }

    #[test]
    fn test_frequency_based_encoding() {
		log::debug!(">>> Entering test_frequency_based_encoding()");
        // In "aaabbc", 'a' should have shorter code than 'b' or 'c'
        let (_, codes) = huffman_encode("aaabbc");
        let a_len = codes[&'a'].len();
		log::debug!("test_frequency_based_encoding(): a_len → {}", a_len);
        let b_len = codes[&'b'].len();
		log::debug!("test_frequency_based_encoding(): b_len → {}", b_len);
        let c_len = codes[&'c'].len();
		log::debug!("test_frequency_based_encoding(): c_len → {}", c_len);

        // 'a' appears most frequently, so should have shortest or equal code
        assert!(a_len <= b_len);
		log::debug!("<<< Exiting test_frequency_based_encoding()");
        assert!(a_len <= c_len);
    }

    #[test]
    fn test_compression_ratio() {
		log::debug!(">>> Entering test_compression_ratio()");
        let text = "aaaaaaaaaa"; // 10 'a's
        let (encoded, _) = huffman_encode(text);
		log::debug!("test_compression_ratio(): text → {}", text);

        // Original: 10 chars * 8 bits = 80 bits (in UTF-8)
        // Huffman: 10 * 1 bit = 10 bits (single character gets code "0")
        assert_eq!(encoded.len(), 10);
		log::debug!("<<< Exiting test_compression_ratio()");
        assert!(encoded.chars().all(|c| c == '0'));
    }

    #[test]
    fn test_all_unique_characters() {
		log::debug!(">>> Entering test_all_unique_characters()");
        let text = "abcdefg";
        let (encoded, codes) = huffman_encode(text);
		log::debug!("test_all_unique_characters(): text → {}", text);

        // All characters should have codes
        assert_eq!(codes.len(), 7);

        // Verify roundtrip
        let decoded = huffman_decode(&encoded, &codes);
		log::debug!("test_all_unique_characters(): decoded → {}", decoded);
		log::debug!("<<< Exiting test_all_unique_characters()");
        assert_eq!(decoded, text);
    }

    #[test]
    fn test_build_frequency_map() {
		log::debug!(">>> Entering test_build_frequency_map()");
        let frequencies = build_frequency_map("hello");
        assert_eq!(frequencies.get(&'h'), Some(&1));
		log::debug!("test_build_frequency_map(): frequencies → {:?}", frequencies);
        assert_eq!(frequencies.get(&'e'), Some(&1));
        assert_eq!(frequencies.get(&'l'), Some(&2));
		log::debug!("<<< Exiting test_build_frequency_map()");
        assert_eq!(frequencies.get(&'o'), Some(&1));
    }

    #[test]
    fn test_unicode_characters() {
		log::debug!(">>> Entering test_unicode_characters()");
        let text = "Hello, 世界! 🌍";
        let (encoded, codes) = huffman_encode(text);
		log::debug!("test_unicode_characters(): text → {}", text);
        let decoded = huffman_decode(&encoded, &codes);
		log::debug!("test_unicode_characters(): decoded → {}", decoded);
		log::debug!("<<< Exiting test_unicode_characters()");
        assert_eq!(decoded, text);
    }

    #[test]
    fn test_demonstrate_huffman_from_file() {
		log::debug!(">>> Entering test_demonstrate_huffman_from_file()");
        use std::fs::File;
        use std::io::Write;

        // Create a temporary test file
        let test_file = "/tmp/huffman_test.txt";
		log::debug!("test_demonstrate_huffman_from_file(): test_file → {}", test_file);
        let test_content = "The quick brown fox jumps over the lazy dog";
		log::debug!("test_demonstrate_huffman_from_file(): test_content → {}", test_content);

        {
            let mut file = File::create(test_file).unwrap();
			log::debug!("test_demonstrate_huffman_from_file(): file → (unprintable)");
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Test the demonstrate function
        let result = demonstrate_huffman_from_file(test_file);
		log::debug!("test_demonstrate_huffman_from_file(): result → {:?}", result);
		log::debug!("<<< Exiting test_demonstrate_huffman_from_file()");
        assert!(result.is_ok());
    }

    #[test]
    fn test_demonstrate_empty_file() {
		log::debug!(">>> Entering test_demonstrate_empty_file()");
        use std::fs::File;

        // Create an empty test file
        let test_file = "/tmp/huffman_empty.txt";
		log::debug!("test_demonstrate_empty_file(): test_file → {}", test_file);
        File::create(test_file).unwrap();

        // Test with empty file
        let result = demonstrate_huffman_from_file(test_file);
		log::debug!("test_demonstrate_empty_file(): result → {:?}", result);
		log::debug!("<<< Exiting test_demonstrate_empty_file()");
        assert!(result.is_ok());
    }
}

/// Main function for command-line usage
///
/// Allows this file to be compiled as a standalone binary:
/// ```bash
/// rustc huffman_encoding.rs -o huffman
/// ./huffman input.txt
/// ```
#[cfg(not(test))]
#[allow(dead_code)]
fn main() {
	log::debug!(">>> Entering main()");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Huffman Encoding - Lossless Data Compression");
        eprintln!();
        eprintln!("Usage: {} <file_path>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} sample.txt", args[0]);
        eprintln!();
        eprintln!("This will encode the file and display:");
        eprintln!("  - Character code mappings");
        eprintln!("  - Compression statistics");
        eprintln!("  - Encoded binary output");
        eprintln!("  - Verification of successful decoding");
        std::process::exit(1);
    }

    let file_path = &args[1];
	log::debug!("<<< Exiting main()");

    match demonstrate_huffman_from_file(file_path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error processing file '{file_path}': {e}");
            std::process::exit(1);
        }
    }
}
