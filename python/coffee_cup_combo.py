n = input()
coffee_machines = input()

coffees = 0
lectures = 0

for machine in coffee_machines: 
    if machine == "0" and coffees > 0:
        coffees -= 1
        lectures += 1

    if machine == "1":
        coffees = 2
        lectures += 1

print(lectures)