impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut total_surplus = 0;
        let mut current_surplus = 0;
        let mut start_station = 0;

        for i in 0..n {
            total_surplus += gas[i] - cost[i];
            current_surplus += gas[i] - cost[i];

            if current_surplus < 0 {
                start_station = i + 1;
                current_surplus = 0;
            }
        }

        if total_surplus >= 0 { start_station as i32 } else { -1 }
    }
}