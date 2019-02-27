#/usr/bin/env python3

import random
import time

equation = lambda x, y, z: abs(x - y - 0.2) * abs(x * z - 0.8) *            \
                           abs(0.3 - z * z * y) + (x * y * (1-z) * abs(z - 0.5))


def generate_neighbor(sp: (float, float, float), r: float) -> (float, float, float):
    z1, z2, z3 = [random.uniform(-abs(r), abs(r)) for _ in range(3)]
    return (sp[0] + z1, sp[1] + z2, sp[2] + z3)

generate_neighbors = lambda sp, r, p: [generate_neighbor(sp, r) for _ in range(p)]

def get_neighbor_val(neighbor: (float, float, float)) -> float:
    return equation(neighbor[0], neighbor[1], neighbor[2])

def get_neighbor_vals(neighbors: [(float, float, float)]) -> [float, ...]:
    vals = []
    for ii in range(len(neighbors)):
        vals.append(get_neighbor_val(neighbors[ii]))
    return vals

def is_better_neighbor(sp: (float, float, float), neighbors: [(float, float, float)]) -> bool:
    sp_val = get_neighbor_val(sp)
    for val in get_neighbor_vals(neighbors):
        if val > sp_val:
            return True
    return False

def get_best_neighbor(sp: (float, float, float),
                      neighbors: [(float, float, float)]) -> (float, float, float):
    """
        This will return the best of the neighbors irregardless of wether or
        not that neighbor is better than the original
    """
    vals = get_neighbor_vals(neighbors)
    best_index, best_val = 0, vals[0]
    #bad name for this function but meh

    for ii in range(len(vals)):
        if vals[ii] > best_val:
            best_val = vals[ii]
            best_index = ii

    return neighbors[best_index]

def RHC(sp: (float, float, float), p: int, r: float, seed: int)             \
                                        -> (int, float, (float, float, float)):
    """
        sp: Starting point of the Hill Climbing Run
        p: number of neighbors of the current solution
        r: neighborhood size
        seed: seed for rng

        returns:
            a tuple containing: the number of solutions that were generated
                                the value of the solution
                                another tuple containing the values of x,y,z
    """
    ii = 0
    while True:
        ii += 1
        neighbors = generate_neighbors(sp, r, p)

        #if we didn't find any better places to move, just stop
        if not is_better_neighbor(sp, neighbors):
            break

        best = get_best_neighbor(sp, neighbors)
        assert(get_neighbor_val(best) > get_neighbor_val(sp))
        sp = best

        if ii % 10000 == 0:
            print(sp, get_neighbor_val(sp))

    print(ii)
    return sp

def run_RHC(sp, p, r, num_runs=5):
    print("sp: {}\np: {}\nz: {}\nnum_runs: {}".format(sp, p, r, num_runs))
    results = []
    for _ in range(num_runs):
        #randomize seed selection to some extent
        random.seed(int(time.time()))
        seed = random.randint(1, 99999999999)
        random.seed(seed)

        print("Using seed: {}, ".format(seed), end="")
        results.append(RHC(sp, p, r, seed))
        print("Result: {}, value: {}\n".format(results[-1],
                                               get_neighbor_val(results[-1])))

    print("\n\n")
    return results

def main(vals=None):
    sps = ((0.5, 0.5, 0.5), (0,0.5,1), (0.9, 0.6, 0.3))
    ps = [20]#, 100)
    rs = (0.02, 0.05)

    if vals:
        for sp in sps:
           run_RHC(sp, vals[0], vals[1])
        exit()

    ii = 0
    for sp in sps:
        for p in ps:
            for r in rs:
                print("Run number: {}".format(ii))
                ii += 1
                run_RHC(sp, p, r)

if __name__ == '__main__':
    import sys
    if len(sys.argv) == 3:
        main((int(sys.argv[1]), float(sys.argv[2])))
    main()
