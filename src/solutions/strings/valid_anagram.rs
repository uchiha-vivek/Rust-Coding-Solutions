use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum AnagramError {
    LengthMismatch,
}

impl fmt::Display for AnagramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnagramError::LengthMismatch => {
                write!(f, "Strings have different lengths")
            }
        }
    }
}









pub struct Solution;

impl Solution {
    pub fn is_anagram(
        s: String,
        t: String,
    ) -> Result<bool, AnagramError> {
        if s.len() != t.len() {
            return Err(AnagramError::LengthMismatch);
        }

        let mut count: HashMap<char, i32> = HashMap::new();

        for ch in s.chars() {
            *count.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            match count.get_mut(&ch) {
                Some(v) => {
                    *v -= 1;
                    if *v == 0 {
                        count.remove(&ch);
                    }
                }
                None => return Ok(false),
            }
        }

        Ok(count.is_empty())
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_anagram() {
        let result = Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string(),
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_invalid_anagram() {
        let result = Solution::is_anagram(
            "rat".to_string(),
            "car".to_string(),
        );
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_length_mismatch() {
        let result = Solution::is_anagram(
            "hello".to_string(),
            "helloo".to_string(),
        );
        assert!(matches!(result, Err(AnagramError::LengthMismatch)));
    }

    #[test]
    fn test_empty_strings() {
        let result = Solution::is_anagram(
            "".to_string(),
            "".to_string(),
        );
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_unicode_anagram() {
        let result = Solution::is_anagram(
            "あい".to_string(),
            "いあ".to_string(),
        );
        assert_eq!(result, Ok(true));
    }
}
