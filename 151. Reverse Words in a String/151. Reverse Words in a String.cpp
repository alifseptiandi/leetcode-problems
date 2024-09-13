class Solution {
public:
    string reverseWords(string s) {
        // Trim leading and trailing spaces
        int left = 0, right = s.length() - 1;
        while (left <= right && s[left] == ' ') left++;
        while (right >= left && s[right] == ' ') right--;
        
        vector<string> words;
        string word;
        
        // Extract words
        for (int i = left; i <= right; i++) {
            if (s[i] != ' ') {
                word += s[i];
            } else if (!word.empty()) {
                words.push_back(word);
                word.clear();
            }
        }
        if (!word.empty()) {
            words.push_back(word);
        }
        
        // Reverse and join words
        reverse(words.begin(), words.end());
        string result;
        for (const auto& w : words) {
            if (!result.empty()) result += " ";
            result += w;
        }
        
        return result;
    }
};