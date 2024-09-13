impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        if n == 0 {
            return 0;
        }
        
        // Step 1: Initialize a candies array where each child gets at least one candy
        let mut candies = vec![1; n];
        
        // Step 2: Left to right pass
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }
        
        // Step 3: Right to left pass
        for i in (0..n-1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }
        
        // Step 4: Calculate the total candies
        candies.iter().sum()
    }
}