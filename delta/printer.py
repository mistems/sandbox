def fib(n: int):
    if n < 0:
        return []
    elif n == 1:
        return [1]
    elif n == 2:
        return [1, 1]
    else:
        ans = [0 for i in range(n)]
        ans[0] = 1
        ans[1] = 1
        for i in range(2, n):
            ans[i] = ans[i-2] + ans[i-1]
        return ans

print(ans(10))