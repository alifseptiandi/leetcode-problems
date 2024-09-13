class Solution {
    public int hIndex(int[] citations) {
        int n = citations.length;
        int[] count = new int[n + 1];
        
        // Count the frequency of each citation count
        for (int c : citations) {
            if (c >= n) {
                count[n]++;
            } else {
                count[c]++;
            }
        }
        
        // Calculate the h-index
        int total = 0;
        for (int i = n; i >= 0; i--) {
            total += count[i];
            if (total >= i) {
                return i;
            }
        }
        
        return 0;
    }

    // For testing purposes
    public static void main(String[] args) {
        Solution solution = new Solution();
        int[] test1 = {3, 0, 6, 1, 5};
        int[] test2 = {1, 3, 1};
        
        System.out.println("H-index for [3,0,6,1,5]: " + solution.hIndex(test1));
        System.out.println("H-index for [1,3,1]: " + solution.hIndex(test2));
    }
}