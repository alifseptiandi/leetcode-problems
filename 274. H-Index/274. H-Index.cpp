#include <vector>

class Solution {
public:
    int hIndex(std::vector<int>& citations) {
        int n = citations.size();
        std::vector<int> count(n + 1, 0);
        
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
};