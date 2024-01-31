project('exercises', 'c')


apps = {
    'ex_1_3': ['ex_1_3.c'],
    'ex_1_4': ['ex_1_4.c'],
    'ex_1_6': ['ex_1_6.c'],
    'ex_1_7': ['ex_1_7.c'],
    'ex_1_8': ['ex_1_8.c'],
    'ex_1_9': ['ex_1_9.c'],
    'ex_1_10': ['ex_1_10.c'],
    'ex_1_12': ['ex_1_12.c'],
}

foreach exname, sources : apps
    executable(exname, join_paths('src', sources))
endforeach