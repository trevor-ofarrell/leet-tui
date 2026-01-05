def multiply(num1, num2):
    if num1 == '0' or num2 == '0':
        return '0'
    result = [0] * (len(num1) + len(num2))
    for i in range(len(num1) - 1, -1, -1):
        for j in range(len(num2) - 1, -1, -1):
            mul = int(num1[i]) * int(num2[j])
            p1, p2 = i + j, i + j + 1
            total = mul + result[p2]
            result[p2] = total % 10
            result[p1] += total // 10
    res = ''.join(map(str, result)).lstrip('0')
    return res or '0'
