import typing

def find_two(values: typing.Set[int], target: int) -> typing.Optional[typing.Set[int]]:
    for val in values:
        if target - val in values:
            return {target-val, val}

def find_three(values: typing.Set[int], target: int) -> typing.Optional[typing.Set[int]]:
    for val in values:
        subset = {x for x in values if x != val}
        if pair := find_two(subset, target - val):
            return pair | {val}
    


values = {1721, 979, 366, 299, 675,1456}
print(find_thee(values, 2020))