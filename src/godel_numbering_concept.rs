
// src/godel_numbering_concept.rs

/// Converts a string into a large unsigned integer (a simplified Gödel numbering concept).
/// This is a conceptual demonstration and not a true Gödel numbering scheme.
/// For simplicity, it concatenates the byte values of the string into a u128.
/// This will overflow for very long strings, but serves the conceptual purpose.
fn string_to_number(s: &str) -> u128 {
    let mut number: u128 = 0;
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        // Shift existing number to make space for the new byte
        // This is a very simplified approach and will quickly overflow for longer strings
        // A true Gödel numbering would involve prime factorization or similar complex methods.
        number = number.saturating_add((byte as u128) << (i * 8));
    }
    number
}

/// Checks a simple property of the generated number.
/// This represents making a "statement" about the embedded document.
fn check_property(n: u128) -> bool {
    // Example property: Is the number even?
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_number() {
        assert_eq!(string_to_number("a"), 97);
        assert_eq!(string_to_number("b"), 98);
        // "ab" -> 0x6261 (little-endian byte order)
        assert_eq!(string_to_number("ab"), 25185);
        // "ba" -> 0x6162 (little-endian byte order)
        assert_eq!(string_to_number("ba"), 24930);
    }

    #[test]
    fn test_check_property() {
        // "a" -> 97 (odd)
        assert_eq!(check_property(string_to_number("a")), false);
        // "b" -> 98 (even)
        assert_eq!(check_property(string_to_number("b")), true);
        // "ab" -> 25185 (odd)
        assert_eq!(check_property(string_to_number("ab")), false);
        // "ba" -> 24930 (even)
        assert_eq!(check_property(string_to_number("ba")), true);
    }
}

fn main() {
    let document_content = "This is a simplified document.";
    let godel_num = string_to_number(document_content);
    println!("Document: "{}"", document_content);
    println!("Simplified Gödel Number: {}", godel_num);

    let is_even = check_property(godel_num);
    println!("Is the simplified Gödel Number even? {}", is_even);

    let another_document = "Another document.";
    let another_godel_num = string_to_number(another_document);
    println!("Document: "{}"", another_document);
    println!("Simplified Gödel Number: {}", another_godel_num);
    println!("Is the simplified Gödel Number even? {}", check_property(another_godel_num));
}
