
from time import sleep
# sleep(100)
IO = lambda: list(map(int, input().split()))
n, target = IO()
arr = [ (value, idx + 1) for idx, value in enumerate(IO()) ]
dic = {}

def solve():
    #print(arr)
    arr.sort()
    l , r = 0, n - 1
    while l < r:
        if arr[l][0] + arr[r][0] == target:
            print(arr[l][1], arr[r][1],end='')
            return
        elif arr[l][0] + arr[r][0]  > target :
            r -= 1
        else:
            l += 1
    print("IMPOSSIBLE")
solve()
