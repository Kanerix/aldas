entries = []
for i in range(int(input())):
    name, grade = input().strip().split()
    suffix_amount = 0

    if grade.startswith("FX"):
        grade = grade[1:]
    elif grade.startswith("F"):
        grade = "Y" + grade[1:]
    else:
        grade = grade
    
    if "+" in grade:
        suffix_amount = -grade.count("+")
    elif "-" in grade:
        suffix_amount = grade.count("-")
    else:
        grade = grade[:1] + ","

    entries.append((name, grade[:2], suffix_amount))

entries.sort(key=lambda entry: (entry[1], entry[2], entry[0]))
print("\n".join([entry[0] for entry in entries]))