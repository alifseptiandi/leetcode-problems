/**
 * @param {number[]} citations
 * @return {number}
 */
function hIndex(citations) {
    const n = citations.length;
    const counts = new Array(n + 1).fill(0);
    
    // Count the number of papers for each citation count
    for (let citation of citations) {
        if (citation >= n) {
            counts[n]++;
        } else {
            counts[citation]++;
        }
    }
    
    // Calculate the h-index
    let total = 0;
    for (let i = n; i >= 0; i--) {
        total += counts[i];
        if (total >= i) {
            return i;
        }
    }
    
    return 0;
}

console.log(hIndex([3,0,6,1,5]));  // Output: 3
console.log(hIndex([1,3,1]));      // Output: 1