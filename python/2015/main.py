inp = """
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
"""

ops = {
    "AND": lambda x, y: x & y,
    "OR": lambda x, y: x | y,
    "LSHIFT": lambda x, y: x << y,
    "RSHIFT": lambda x, y: x >> y,
    "NOT": lambda x: ~x,
}

# read string line by line
# for each line, there is the form:
# NUM -> reg (put number into register)
# reg OP reg -> reg (binary bitwise op two reg into reg)
# OP reg -> reg (unary op reg into reg)
# reg SHIFT num -> reg (shift register by some number into reg)
for l in inp.splitlines():
    print(l)
