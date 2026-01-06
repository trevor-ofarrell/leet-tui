class Solution {
public:
    string multiply(string num1, string num2) {
        if (num1 == "0" || num2 == "0") {
            return "0";
        }
        int n1 = num1.size(), n2 = num2.size();
        vector<int> result(n1 + n2, 0);

        for (int i = n1 - 1; i >= 0; i--) {
            for (int j = n2 - 1; j >= 0; j--) {
                int mul = (num1[i] - '0') * (num2[j] - '0');
                int p1 = i + j, p2 = i + j + 1;
                int total = mul + result[p2];
                result[p2] = total % 10;
                result[p1] += total / 10;
            }
        }

        string res;
        for (int digit : result) {
            if (!(res.empty() && digit == 0)) {
                res += to_string(digit);
            }
        }
        return res.empty() ? "0" : res;
    }
};
