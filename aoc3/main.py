# i'll fight with rust at a later date, but this is an ugly way of doing this

with open('./input', newline='') as input:
    prev_num = []
    prev_sym = []

    total = 0
    lines = input.readlines()
    for line in lines:
        curr_num = []
        curr_sym = []
        is_prev_num = False     # is the last symbol read a number
        num = ''
        start_pos = 0
        for i in range(len(line)):
            if line[i].isdigit():
                if not is_prev_num: start_pos = i
                num += line[i]
                is_prev_num = True
            else:
                if line[i] != '.' and line[i] != '\n': curr_sym.append(i)
                if is_prev_num:
                    curr_num.append({'num': int(num),
                                     'start': start_pos,
                                     'end': i-1,
                                     })
                    is_prev_num = False
                num = ''

        for pos in prev_sym:
            for num in curr_num:
                if pos >= num['start']-1 and pos <= num['end']+1:
                    total += num['num']

        for pos in curr_sym:
            for num in curr_num:
                if pos >= num['start']-1 and pos <= num['end']+1:
                    total += num['num']
            for num in prev_num:
                if pos >= num['start']-1 and pos <= num['end']+1:
                    total += num['num']

        prev_sym = curr_sym
        prev_num = curr_num
    print('one star total = ', total)

def is_equal_dict(dic1, dic2):
    if dic1['num'] != dic2['num']: return False
    if dic1['pos'][0] != dic2['pos'][0]: return False
    if dic1['pos'][1] != dic2['pos'][1]: return False
    return True

def is_equal_list(l1, l2):
    if len(l1) != len(l2): return False
    for i in range(len(l1)):
        if l1[i] != l2[i]: return False
    return True

def is_in(ls, dic):
    for l in ls:
        if is_equal_dict(l, dic): return True
    return False

with open('./input', newline='') as input:
    prev1_num = []
    prev1_sym = []
    prev2_num = []
    prev2_sym = []
    prev_adj = []

    total = 0
    lines = input.readlines()
    for line in lines:
        curr_num = []
        curr_sym = []
        is_prev_num = False
        num = ''
        start_pos = 0
        for i in range(len(line)):
            if line[i].isdigit():
                if not is_prev_num: start_pos = i
                num += line[i]
                is_prev_num = True
            else:
                if line[i] == '*': curr_sym.append(i)
                if is_prev_num:
                    curr_num.append({'num': int(num),
                                     'start': start_pos,
                                     'end': i-1,
                                     })
                    is_prev_num = False
                num = ''

        adj_num = []
        for pos in prev2_sym:
            for num in prev1_num:
                if pos >= num['start']-1 and pos <= num['end']+1:
                    adj_num.append({'num': num['num'], 'pos': [2,pos]})
            for num in prev2_num:
                if pos >= num['start']-1 and pos <= num['end']+1:
                    adj_num.append({'num': num['num'], 'pos': [2,pos]})

        for pos in prev1_sym:
            for num in curr_num:
                if pos >= num['start']-1 and pos <= num['end']+1:
                    adj_num.append({'num': num['num'], 'pos': [1,pos]})
            for num in prev1_num:
                if pos >= num['start']-1 and pos <= num['end']+1:
                    adj_num.append({'num': num['num'], 'pos': [1,pos]})
            for num in prev2_num:
                if pos >= num['start']-1 and pos <= num['end']+1:
                    adj_num.append({'num': num['num'], 'pos': [1,pos]})

        for pos in curr_sym:
            for num in curr_num:
                if pos >= num['start']-1 and pos <= num['end']+1:
                    adj_num.append({'num': num['num'], 'pos': [0,pos]})
            for num in prev1_num:
                if pos >= num['start']-1 and pos <= num['end']+1:
                    adj_num.append({'num': num['num'], 'pos': [0,pos]})

        adj_num_helper = adj_num.copy()
        for gear1 in adj_num:
            number_of_adj = 1
            gear_ratio = gear1['num']
            other_gear = []
            for gear2 in adj_num:
                if not is_equal_dict(gear1, gear2) and is_equal_list(gear1['pos'], gear2['pos']) and (not is_in(prev_adj, gear1) or not is_in(prev_adj, gear2)):
                    number_of_adj += 1
                    gear_ratio *= gear2['num']
                    other_gear = gear2
            if number_of_adj == 2:
                total += gear_ratio
            adj_num.remove(gear1)

        prev2_sym = prev1_sym
        prev2_num = prev1_num
        prev1_sym = curr_sym
        prev1_num = curr_num
        prev_adj = []
        for gear in adj_num_helper:
            if gear['pos'][0] != 2:
                prev_adj.append({'num': gear['num'],
                                 'pos': [gear['pos'][0]+1,gear['pos'][1]]
                    })
    print('two star total = ', total)
