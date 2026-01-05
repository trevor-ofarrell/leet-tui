def isValid(s):
    stack = []
    mapping = {')': '(', '}': '{', ']': '['}
    opens = {'(', '{', '['}
    for c in s:
        if c in mapping:
            if not stack or stack.pop() != mapping[c]:
                return False
        elif c in opens:
            stack.append(c)
    return len(stack) == 0
