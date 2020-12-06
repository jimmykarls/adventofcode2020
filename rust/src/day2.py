import re


pattern = r"([0-9]*)-([0-9]*)\s([a-z]*):\s([a-z]*)"
with open("input_day2", "r") as f:
    counter = 0
    for line in f.readlines():
        match = re.match(pattern, line)

        if not match:
            continue

        first, second, letter, pw = match.groups()

        f = int(first) - 1
        s = int(second) - 1

        match_counter = 0

        match_counter += 1 if pw[f] == letter else 0
        match_counter += 1 if pw[s] == letter else 0

        if match_counter == 1:
            counter += 1
    
    print(counter)