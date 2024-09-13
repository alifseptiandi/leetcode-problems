impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let (mut s_idx, mut t_idx) = (0, 0);

        while s_idx < s_chars.len() && t_idx < t_chars.len() {
            if s_chars[s_idx] == t_chars[t_idx] {
                s_idx += 1;
            }
            t_idx += 1;
        }

        s_idx == s_chars.len()
    }
}