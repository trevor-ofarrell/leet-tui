class Solution {
public:
    bool checkInclusion(string s1, string s2) {
        if (s1.length() > s2.length()) return false;

        vector<int> count(26, 0);
        for (int i = 0; i < s1.length(); i++) {
            count[s1[i] - 'a']++;
            count[s2[i] - 'a']--;
        }

        if (allZero(count)) return true;

        for (int i = s1.length(); i < s2.length(); i++) {
            count[s2[i] - 'a']--;
            count[s2[i - s1.length()] - 'a']++;
            if (allZero(count)) return true;
        }
        return false;
    }

private:
    bool allZero(vector<int>& count) {
        for (int c : count) {
            if (c != 0) return false;
        }
        return true;
    }
};
