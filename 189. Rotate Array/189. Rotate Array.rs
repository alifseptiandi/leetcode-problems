impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        if n <= 1 {
            return;  // No rotation needed for empty or single-element arrays
        }
        
        let k = k as usize % n;  // Handle cases where k > n
        if k == 0 {
            return;  // No rotation needed when k is 0 or a multiple of n
        }
        
        // Reverse the entire array
        Self::reverse(nums, 0, n - 1);
        // Reverse the first k elements
        Self::reverse(nums, 0, k - 1);
        // Reverse the remaining n-k elements
        Self::reverse(nums, k, n - 1);
    }
    
    fn reverse(nums: &mut Vec<i32>, start: usize, end: usize) {
        let mut i = start;
        let mut j = end;
        while i < j {
            nums.swap(i, j);
            i += 1;
            j = j.saturating_sub(1);  // Prevent underflow
        }
    }
}