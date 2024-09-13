impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // Convert to lowercase and filter out non-alphanumeric characters
        let chars: Vec<char> = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        // Check if the string is a palindrome
        chars.iter().eq(chars.iter().rev())
    }
}

// The struct Solution is already defined in the problem framework
// No need to define it here