class Solution {
    /**
     * @param Integer[] $nums
     * @param Integer $target
     * @return Integer[]
     */
    function twoSum($nums, $target) {
        $numMap = array();
        
        foreach ($nums as $i => $num) {
            $complement = $target - $num;
            if (isset($numMap[$complement])) {
                return array($numMap[$complement], $i);
            }
            $numMap[$num] = $i;
        }
        
        // This should not happen given the problem constraints
        return array();
    }
}