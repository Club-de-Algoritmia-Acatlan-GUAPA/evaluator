from time import sleep
import sys

def a():
    n = int(input())
    _ = input()
    for _ in range(n):
        arr = list(map(int, input().split()))
        print(arr)
a()  
