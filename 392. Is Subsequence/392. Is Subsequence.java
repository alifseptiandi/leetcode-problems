import java.util.*;

class Solution {
    private Map<Character, List<Integer>> charIndices;

    public boolean isSubsequence(String s, String t) {
        // Preprocess t
        preprocessT(t);
        
        int lastIndex = -1;
        for (char c : s.toCharArray()) {
            if (!charIndices.containsKey(c)) {
                return false;
            }
            
            List<Integer> indices = charIndices.get(c);
            int pos = Collections.binarySearch(indices, lastIndex + 1);
            if (pos < 0) pos = -pos - 1;
            if (pos == indices.size()) return false;
            
            lastIndex = indices.get(pos);
        }
        
        return true;
    }

    private void preprocessT(String t) {
        charIndices = new HashMap<>();
        for (int i = 0; i < t.length(); i++) {
            charIndices.computeIfAbsent(t.charAt(i), k -> new ArrayList<>()).add(i);
        }
    }
}