impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];
        
        // Calculate prefix products
        let mut prefix = 1;
        for i in 0..n {
            result[i] *= prefix;
            prefix *= nums[i];
        }
        
        // Calculate suffix products and combine with prefix products
        let mut suffix = 1;
        for i in (0..n).rev() {
            result[i] *= suffix;
            suffix *= nums[i];
        }
        
        result
    }
}