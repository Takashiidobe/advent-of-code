def get_input():
    with open('day_02.txt') as f:
        return [line.strip() for line in f.readlines()]

def parse_input():
    return [(l.split(' ')[0], l.split(' ')[1]) for l in get_input()]

shape_points = {
    ord('A'): 1,
    ord('B'): 2,
    ord('C'): 3
}

outcome_points = {
    'Win': 6,
    'Draw': 3,
    'Loss': 0,
}

def part_1(inp):
    curr_score = 0

    inp = map(lambda x: (ord(x[0]), ord(x[1]) - 23), inp)

    for opp_choice, your_choice in inp:
        draw = opp_choice == your_choice
        win = opp_choice == ord('A') and your_choice == ord('B') or \
                opp_choice == ord('B') and your_choice == ord('C') or \
                opp_choice == ord('C') and your_choice == ord('A')
        loss = not win and not draw

        if draw:
            curr_score += outcome_points['Draw']
        elif win:
            curr_score += outcome_points['Win']
        elif loss:
            curr_score += outcome_points['Loss']
        curr_score += shape_points[your_choice]
    return curr_score

def part_2(inp):
    win_choice = {
        'A': 'Y',
        'B': 'Z',
        'C': 'X'
    }
    lose_choice = {
        'A': 'Z',
        'B': 'X',
        'C': 'Y'
    }
    corrected = []
    for opp_choice, outcome in inp:
        if outcome == 'X':
            your_choice = lose_choice[opp_choice]
        if outcome == 'Y':
            your_choice = chr(ord(opp_choice) + 23)
        if outcome == 'Z':
            your_choice = win_choice[opp_choice]
        corrected.append((opp_choice, your_choice))

    return part_1(corrected)

print(part_2(parse_input()))
