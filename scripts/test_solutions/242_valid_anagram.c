bool isAnagram(char* s, char* t) {
    int count[26] = {0};
    int sLen = strlen(s);
    int tLen = strlen(t);
    if (sLen != tLen) return false;
    for (int i = 0; i < sLen; i++) {
        count[s[i] - 'a']++;
        count[t[i] - 'a']--;
    }
    for (int i = 0; i < 26; i++) {
        if (count[i] != 0) return false;
    }
    return true;
}
