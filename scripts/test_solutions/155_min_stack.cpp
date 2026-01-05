class MinStack {
private:
    vector<int> stack;
    vector<int> min_stack;

public:
    MinStack() {
    }

    void push(int val) {
        stack.push_back(val);
        int min_val = min_stack.empty() ? val : min(val, min_stack.back());
        min_stack.push_back(min_val);
    }

    void pop() {
        if (!stack.empty()) {
            stack.pop_back();
            min_stack.pop_back();
        }
    }

    int top() {
        if (stack.empty()) return INT_MAX;  // sentinel value for empty stack
        return stack.back();
    }

    int getMin() {
        if (min_stack.empty()) return INT_MAX;  // sentinel value for empty stack
        return min_stack.back();
    }
};
