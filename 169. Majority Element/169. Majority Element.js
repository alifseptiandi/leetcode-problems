function majorityElement(nums) {
    let count = 0;
    let candidate;

    for (let num of nums) {
        if (count === 0) {
            candidate = num;
            count = 1;
        } else if (candidate === num) {
            count++;
        } else {
            count--;
        }
    }

    return candidate;
}

// Example usage:
let nums = [3, 2, 3];
console.log(majorityElement(nums)); // Output: 3

nums = [2, 2, 1, 1, 1, 2, 2];
console.log(majorityElement(nums)); // Output: 2