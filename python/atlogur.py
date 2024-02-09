n = int(input())

knights = []

for i in range(n):
    line = input()
    
    h,s = line.split()
    
    h = int(h)
    s = int(s)
    
    knights.append([h,s,i])

while len(knights) > 1:
    attack = knights[0]
    defend = knights[1]
    
    defend[0] -= attack[1]
    if defend[0] <= 0:
        knights.pop(1)
        continue
        
    attack[0] -= defend[1]
    if attack[0] <= 0:
        knights.pop(0)
        continue

print(knights[0][2] + 1)