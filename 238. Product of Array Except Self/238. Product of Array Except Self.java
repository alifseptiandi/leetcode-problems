class Solution {
    public int[] productExceptSelf(int[] nums) {
        int n = nums.length;
        int[] result = new int[n];
        
        // Initialize result array with 1s
        Arrays.fill(result, 1);
        
        // Calculate prefix products
        int prefix = 1;
        for (int i = 0; i < n; i++) {
            result[i] *= prefix;
            prefix *= nums[i];
        }
        
        // Calculate suffix products and combine with prefix products
        int suffix = 1;
        for (int i = n - 1; i >= 0; i--) {
            result[i] *= suffix;
            suffix *= nums[i];
        }
        
        return result;
    }
}