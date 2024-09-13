class Solution {
private:
    unordered_map<char, vector<int>> charIndices;

    void preprocessT(const string& t) {
        for (int i = 0; i < t.length(); i++) {
            charIndices[t[i]].push_back(i);
        }
    }

public:
    bool isSubsequence(string s, string t) {
        // Preprocess t
        preprocessT(t);
        
        int lastIndex = -1;
        for (char c : s) {
            if (charIndices.find(c) == charIndices.end()) {
                return false;
            }
            
            const vector<int>& indices = charIndices[c];
            auto it = upper_bound(indices.begin(), indices.end(), lastIndex);
            if (it == indices.end()) return false;
            
            lastIndex = *it;
        }
        
        return true;
    }
};