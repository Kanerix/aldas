n = input()
cof_machines = input()

cof = 0
lec = 0

for machine in cof_machines: 
    if machine == "0" and cof > 0:
        cof -= 1
        lec += 1

    if machine == "1":
        cof = 2
        lec += 1

print(lec)