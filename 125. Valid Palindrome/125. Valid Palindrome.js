function isPalindrome(s) {
    // Step 1: Normalize the string
    // Convert to lowercase and remove non-alphanumeric characters
    const normalized = s.toLowerCase().replace(/[^a-z0-9]/g, '');

    // Step 2: Check for palindrome
    // Compare the normalized string with its reverse
    const reversed = normalized.split('').reverse().join('');
    return normalized === reversed;
}

// Example usage:
console.log(isPalindrome("A man, a plan, a canal: Panama")); // Output: true
console.log(isPalindrome("race a car")); // Output: false
console.log(isPalindrome(" ")); // Output: true