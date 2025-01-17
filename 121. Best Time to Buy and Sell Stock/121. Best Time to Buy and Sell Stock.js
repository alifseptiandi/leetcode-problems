function maxProfit(prices) {
    if (prices.length === 0) return 0;

    let minPrice = prices[0];
    let maxProfit = 0;

    for (let i = 1; i < prices.length; i++) {
        if (prices[i] < minPrice) {
            minPrice = prices[i];
        } else if (prices[i] - minPrice > maxProfit) {
            maxProfit = prices[i] - minPrice;
        }
    }

    return maxProfit;
}

// Example usage:
let prices = [7, 1, 5, 3, 6, 4];
console.log(maxProfit(prices)); // Output: 5

prices = [7, 6, 4, 3, 1];
console.log(maxProfit(prices)); // Output: 0