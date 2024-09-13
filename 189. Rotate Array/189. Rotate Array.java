class Solution {
    public void rotate(int[] nums, int k) {
        if (nums == null || nums.length <= 1) {
            return;  // No rotation needed for null, empty, or single-element arrays
        }
        
        int n = nums.length;
        k = k % n;  // Handle cases where k > n
        if (k == 0) {
            return;  // No rotation needed when k is 0 or a multiple of n
        }
        
        // Reverse the entire array
        reverse(nums, 0, n - 1);
        // Reverse the first k elements
        reverse(nums, 0, k - 1);
        // Reverse the remaining n-k elements
        reverse(nums, k, n - 1);
    }
    
    private void reverse(int[] nums, int start, int end) {
        while (start < end) {
            int temp = nums[start];
            nums[start] = nums[end];
            nums[end] = temp;
            start++;
            end--;
        }
    }
}