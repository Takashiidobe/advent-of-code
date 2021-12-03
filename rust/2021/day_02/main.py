with open("input.txt", "r") as file:
    steps = [row.rstrip() for row in file]
    # Part 2
    pos_x = 0
    pos_y = 0
    aim = 0
    for step in steps:
        direction, magnitude = step.split()
        magnitude = int(magnitude)
        if direction == "forward":
            pos_x += magnitude
            pos_y += aim * magnitude
        elif direction == "up":
            aim -= magnitude
        elif direction == "down":
            aim += magnitude
        else:
            print(f"{direction} is an unknown direction!")

    print(pos_x * pos_y)
