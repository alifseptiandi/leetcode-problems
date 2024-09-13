function canCompleteCircuit(gas, cost) {
    const n = gas.length;
    let totalGas = 0;
    let totalCost = 0;
    let currentBalance = 0;
    let startIndex = 0;
  
    for (let i = 0; i < n; i++) {
      totalGas += gas[i];
      totalCost += cost[i];
      currentBalance += gas[i] - cost[i];
      
      if (currentBalance < 0) {
        // If the current balance is negative, we cannot start from `startIndex`
        // Set the next station as the new startIndex
        startIndex = i + 1;
        currentBalance = 0; // Reset the current balance
      }
    }
  
    // If total gas is less than total cost, return -1
    return totalGas < totalCost ? -1 : startIndex;
  }
  
  // Example usage:
  console.log(canCompleteCircuit([1, 2, 3, 4, 5], [3, 4, 5, 1, 2])); // Output: 3
  console.log(canCompleteCircuit([2, 3, 4], [3, 4, 3])); // Output: -1