function rotate(nums, k) {
    // Approach 1: Using Reversal Algorithm
    k %= nums.length;
    reverse(nums, 0, nums.length - 1);
    reverse(nums, 0, k - 1);
    reverse(nums, k, nums.length - 1);

    // Approach 2: Using Three Reversals
    // reverse(nums, 0, nums.length - 1);
    // reverse(nums, 0, k - 1);
    // reverse(nums, k, nums.length - 1);

    // Approach 3: Using Modular Arithmetic
    // let n = nums.length;
    // let count = 0;
    // for (let i = 0; i < n; i++) {
    //     let j = i;
    //     while (true) {
    //         j = (j + k) % n;
    //         if (j === i) break;
    //         [nums[i], nums[j]] = [nums[j], nums[i]];
    //         count++;
    //     }
    // }
}

function reverse(arr, start, end) {
    while (start < end) {
        let temp = arr[start];
        arr[start] = arr[end];
        arr[end] = temp;
        start++;
        end--;
    }
}

// Example usage:
let nums = [1, 2, 3, 4, 5, 6, 7];
let k = 3;
rotate(nums, k);
console.log(nums); // Output: [5,6,7,1,2,3,4]

// Example usage:
nums = [1, 2, 3, 4, 5, 6, 7];
k = 3;
rotate(nums, k);
console.log(nums); // Output: [5,6,7,1,2,3,4]

// Example usage:
nums = [1, 2, 3, 4, 5, 6, 7];
k = 3;
rotate(nums, k);
console.log(nums); // Output: [5,6,7,1,2,3,4]