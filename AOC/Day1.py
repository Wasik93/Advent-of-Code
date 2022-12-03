fp = open("Day1input.txt")


c = fp.read()
if c is None:
    print ("bajo jajo")
tab = c.split('\n')
max = int(0)
sum = int(0)
index = int(0)
tab.append('')
kalorie = []
for i in tab:
    if i == '':
        if sum > max:
            max = sum
        kalorie.append(sum)
        sum = int(0)
    else:
        sum = sum + int(i)
kalorie.sort(reverse=True)
print(kalorie[0] + kalorie[1] + kalorie[2])
print(max)