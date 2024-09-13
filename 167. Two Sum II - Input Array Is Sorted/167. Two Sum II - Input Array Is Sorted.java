class Solution {
    public int[] twoSum(int[] numbers, int target) {
        int left = 0;
        int right = numbers.length - 1;
        
        while (left < right) {
            int sum = numbers[left] + numbers[right];
            
            if (sum == target) {
                // We add 1 to convert from 0-indexed to 1-indexed
                return new int[]{left + 1, right + 1};
            } else if (sum < target) {
                left++;
            } else {
                right--;
            }
        }
        
        // This line should never be reached given the problem constraints
        return new int[]{};
    }
}

// Example usage and tests
class Main {
    public static void main(String[] args) {
        Solution solution = new Solution();
        
        // Test case 1
        int[] result1 = solution.twoSum(new int[]{2, 7, 11, 15}, 9);
        System.out.println("Test case 1: [" + result1[0] + ", " + result1[1] + "]");
        
        // Test case 2
        int[] result2 = solution.twoSum(new int[]{2, 3, 4}, 6);
        System.out.println("Test case 2: [" + result2[0] + ", " + result2[1] + "]");
        
        // Test case 3
        int[] result3 = solution.twoSum(new int[]{-1, 0}, -1);
        System.out.println("Test case 3: [" + result3[0] + ", " + result3[1] + "]");
    }
}