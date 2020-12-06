
TREE = "#"

with open("input_day3", "r") as f:
    m = []
    for line in f.readlines():
        m.append(line.strip("\n"))
    
    WIDTH = len(m[0])
    
    col = 0
    tree_counter = 0
    
    for row in range(1, len(m)):
        col = (col + 3) % WIDTH
        tree_counter += 1 if m[row][col] is TREE else 0
    print(tree_counter)

# Read line by line instead of loading the whole file into memory
with open("input_day3", "r") as f:
    l = f.readline() # scrap first row
    col = 0
    tree_counter = 0
    for line in f.readlines():
        line = line.strip("\n")
        col = (col + 3) % WIDTH
        tree_counter += 1 if line[col] is TREE else 0
    print("Second tree counter", tree_counter)


