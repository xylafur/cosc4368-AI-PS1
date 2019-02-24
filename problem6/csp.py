"""
    Our recursive function will return True if a solution is found, False if it
    is not

    if we have no more unallocated varaibles, check the mapping
        return either true or false

        if we return true, print out the mapping

    make a copy of the unallocated values that we will splice

loop:
    pick the next variable from the unallocated varaibles
    pick the next value from the unallocated values copy
    update the mapping

    call the csp function with the unallocated variables, minus the one that we
    popped off for this iteration and the unallocated values minus the value
    that we are using for this particular mapping

    if the call returns false, try the next value

    otherwise return true
"""

variables = ["L", "E", "T", "C", "A", "H", "O", "M"]
values = list(range(0, 10))

###############################################################################
#   This class is just for ease of use, so I don't have to manually calculate
#   the constraints inside of my CSP
###############################################################################
class Constraint:
    def __init__(self, result, variable1=None, variable2=None, carry=None):
        assert(result)
        assert(isinstance(result, str))

        if variable1:
            assert(isinstance(variable1, str))
        if variable2:
            assert(isinstance(variable2, str))

        if not variable1 or not variable2:
            # Can't have just one variable in this particular problem
            assert(variable1 is None)
            assert(variable2 is None)
            #we MUST have a carry if we don't have any varaibles in this problem
            assert(not carry is None)

        if carry:
            assert(isinstance(carry, Constraint))

        self.v1 = variable1
        self.v2 = variable2
        self.r = result
        self.carry = carry

    def __repr__(self):
        if self.v1 and self.carry:
            return "{} + {} + {} = {}".format(self.v1, self.v2, "carry", self.r)

        if self.v1:
            return "{} + {} = {}".format(self.v1, self.v2, self.r)

        return "{} = {}".format("carry", self.r)

    def all_mapped(self, mapping):
        if not self.v1:
            return self.r in mapping and self.carry.all_mapped(mapping)

        if self.v1 not in mapping or self.v2 not in mapping or              \
           self.r not in mapping:
            return False

        if self.carry and not self.carry.all_mapped(mapping):
            return False

        return True

    def get_carry(self, mapping):
        #this function should not be called if we don't have variables
        assert(not self.v1 is None and not self.v2 is None)
        assert(self.all_mapped(mapping))

        carry = self.carry.get_carry(mapping) if self.carry else 0

        return (mapping[self.v1] + mapping[self.v2] + carry) // 10

    def calculate_solution(self, mapping):
        c = self.carry.get_carry(mapping) if self.carry else 0

        return mapping[self.v1] + mapping[self.v2] + c if self.v1 else c

    def constraint_satisfied(self, mapping):
        # This function should never be called unless all of the variables have
        # been mapped
        assert(self.all_mapped(mapping))

        if self.carry:
            assert(self.carry.all_mapped(mapping))
        return mapping[self.r] == self.calculate_solution(mapping)


###############################################################################
#   Make a list of all of the constraints.
#   In the CSP, we will check that each of these are satisfied
###############################################################################
constraints = [
    Constraint("E", variable1="T", variable2="T"),
]
constraints.append(Constraint("M", variable1="E", variable2="A",
                   carry=constraints[-1]))
constraints.append(Constraint("O", variable1="L", variable2="C",
                   carry=constraints[-1]))
constraints.append(Constraint("H", carry=constraints[-1]))

###############################################################################
#   This is the exact same thing as the pseudo code.  The Constraint class make
#   everything significantly more readable
###############################################################################
def csp(unallocated_variables, unallocated_values, mapping):
    if not unallocated_variables:
        # find out if all of the constraints are satisfied and return the result
        all_satisfied = all([each.constraint_satisfied(mapping) for each in
                             constraints])
        if all_satisfied:
            print(mapping)
        return all_satisfied

    variable = unallocated_variables.pop(0)

    values_to_use = unallocated_values[:]

    for value in values_to_use:
        mapping[variable] = value

        if csp(unallocated_variables[:],
               [each for each in unallocated_values[:] if each != value],
               mapping):
            return True

    return False

csp(variables[:], values[:], {})
