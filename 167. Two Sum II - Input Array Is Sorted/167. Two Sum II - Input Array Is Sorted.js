function twoSum(numbers, target) {
    let left = 0;
    let right = numbers.length - 1;

    while (left < right) {
        const sum = numbers[left] + numbers[right];
        if (sum === target) {
            return [left + 1, right + 1]; // Return 1-based indices
        } else if (sum < target) {
            left++; // Move left pointer to the right
        } else {
            right--; // Move right pointer to the left
        }
    }

    // The problem guarantees exactly one solution, so no need for a return here
}

// Example usage:
console.log(twoSum([2, 7, 11, 15], 9)); // Output: [1, 2]
console.log(twoSum([2, 3, 4], 6)); // Output: [1, 3]
console.log(twoSum([-1, 0], -1)); // Output: [1, 2]