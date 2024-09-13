#include <vector>
#include <iostream>

class Solution {
public:
    std::vector<int> twoSum(const std::vector<int>& numbers, int target) {
        int left = 0;
        int right = numbers.size() - 1;
        
        while (left < right) {
            int sum = numbers[left] + numbers[right];
            
            if (sum == target) {
                // We add 1 to convert from 0-indexed to 1-indexed
                return {left + 1, right + 1};
            } else if (sum < target) {
                left++;
            } else {
                right--;
            }
        }
        
        // This line should never be reached given the problem constraints
        return {};
    }
};

// Function to run the solution and print results
void runSolution() {
    Solution solution;
    
    // Test case 1
    std::vector<int> result1 = solution.twoSum({2, 7, 11, 15}, 9);
    std::cout << "Test case 1: [" << result1[0] << ", " << result1[1] << "]\n";
    
    // Test case 2
    std::vector<int> result2 = solution.twoSum({2, 3, 4}, 6);
    std::cout << "Test case 2: [" << result2[0] << ", " << result2[1] << "]\n";
    
    // Test case 3
    std::vector<int> result3 = solution.twoSum({-1, 0}, -1);
    std::cout << "Test case 3: [" << result3[0] << ", " << result3[1] << "]\n";
}

// The main function is likely already defined in your environment.
// You can call runSolution() from the existing main function.