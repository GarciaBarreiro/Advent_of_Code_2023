import re

with open('./input', newline='') as input:
    lines = input.readlines()
    tot_score = 0
    for line in lines:
        x = re.split('\s', line)
        i = 1
        while x[i] == '':
            i+=1
        id = int(re.sub(':','',x[i]))
        winning_nums = []
        our_nums = []
        i += 1
        while x[i] != '|':
            if x[i]: winning_nums.append(int(x[i]))
            i+=1
        i+=1
        while i < len(x):
            if x[i]: our_nums.append(int(x[i]))
            i+=1
        number_win_nums = len([item for item in our_nums if item in winning_nums])
        if number_win_nums > 0:
            tot_score += 2**(number_win_nums-1)
    print("one star score: ", tot_score)

with open('./input', newline='') as input:
    lines = input.readlines()
    tot_scrathcards = 0
    to_repeat = []
    for line in lines:
        x = re.split('\s', line)
        i = 1
        while x[i] == '':
            i+=1
        id = int(re.sub(':','',x[i]))
        winning_nums = []
        our_nums = []
        i += 1
        while x[i] != '|':
            if x[i]: winning_nums.append(int(x[i]))
            i+=1
        i+=1
        while i < len(x):
            if x[i]: our_nums.append(int(x[i]))
            i+=1
        number_win_nums = len([item for item in our_nums if item in winning_nums])
        for i in range(to_repeat.count(id)+1):
            tot_scrathcards += number_win_nums
            for x in range(number_win_nums):
                to_repeat.append(x+id+1)
        to_repeat = [item for item in to_repeat if item != id]
    print("two star score: ", tot_scrathcards+id)
