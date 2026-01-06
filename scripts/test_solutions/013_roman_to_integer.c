int romanToInt(char* s) {
    int values[256] = {0};
    values['I'] = 1;
    values['V'] = 5;
    values['X'] = 10;
    values['L'] = 50;
    values['C'] = 100;
    values['D'] = 500;
    values['M'] = 1000;

    int result = 0;
    int i = 0;
    while (s[i] != '\0') {
        if (s[i + 1] != '\0' && values[(unsigned char)s[i]] < values[(unsigned char)s[i + 1]]) {
            result -= values[(unsigned char)s[i]];
        } else {
            result += values[(unsigned char)s[i]];
        }
        i++;
    }
    return result;
}
