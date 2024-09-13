class Solution {
    public String reverseWords(String s) {
        // Trim leading and trailing spaces
        s = s.trim();
        
        // Split the string into words
        String[] words = s.split("\\s+");
        
        // Reverse the array of words
        for (int i = 0; i < words.length / 2; i++) {
            String temp = words[i];
            words[i] = words[words.length - 1 - i];
            words[words.length - 1 - i] = temp;
        }
        
        // Join the words with a single space
        return String.join(" ", words);
    }
}