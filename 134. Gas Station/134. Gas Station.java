class Solution {
    public int canCompleteCircuit(int[] gas, int[] cost) {
        int n = gas.length;
        int totalSurplus = 0;
        int currentSurplus = 0;
        int startStation = 0;

        for (int i = 0; i < n; i++) {
            totalSurplus += gas[i] - cost[i];
            currentSurplus += gas[i] - cost[i];

            if (currentSurplus < 0) {
                startStation = i + 1;
                currentSurplus = 0;
            }
        }

        return totalSurplus >= 0 ? startStation : -1;
    }
}