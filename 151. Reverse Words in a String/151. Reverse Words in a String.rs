impl Solution {
    pub fn reverse_words(s: String) -> String {
        // Split the string into words, reverse the order, and join with a single space
        s.split_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}