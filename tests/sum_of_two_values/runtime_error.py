a,tar= [int(x) for x in raw_input().split(' ')]
arr = [int(x) for x in raw_input().split(' ')]
mapa = {}
for ( i , x) in enumerate(arr):
    mapa[x] = i
for (i,x) in enumerate(arr):
    if tar - x in mapa and mapa[tar-x] != i:
        print i + 1, mapa[tar-x] + 1
        exit()
print("IMPOSSIBLE")