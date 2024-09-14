class Solution {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        val numMap = mutableMapOf<Int, Int>()
        
        for ((i, num) in nums.withIndex()) {
            val complement = target - num
            if (numMap.containsKey(complement)) {
                return intArrayOf(numMap[complement]!!, i)
            }
            numMap[num] = i
        }
        
        // This should not happen given the problem constraints
        return intArrayOf()
    }
}