def generateParenthesis(n):
    result = []

    def backtrack(open_count, close_count, path):
        if len(path) == 2 * n:
            result.append(path)
            return
        if open_count < n:
            backtrack(open_count + 1, close_count, path + '(')
        if close_count < open_count:
            backtrack(open_count, close_count + 1, path + ')')

    backtrack(0, 0, '')
    return result
