import sys
from rhc import get_neighbor_val
ii =1
_file = sys.argv[1]
out = sys.argv[2]
with open(_file) as f:
    with open(out, 'w') as f2:
        for line in f:
            f2.write(line)
            if "Result: " in line:
                val = line[7:]
                val = tuple([float(e.strip()) for e in val.replace('(', '').replace(')','').split(',')])
                val = get_neighbor_val(val)

                f2.write("f{} = {}\n".format(line[7:].strip(), val))

                if ii % 5 == 0:
                    print()

                ii += 1



