#!/usr/bin/python
'''
This parser converts python file .py into abstract syntax trees
'''
import ast
import os
import sys

# check for command line arguments:
if len(sys.argv) == 1:
    print('No input file specified!\nExiting...')
    exit(1)
else: # passed one or more files
    print(len(sys.argv) - 1, 'file(s):')
    ITERARGS = iter(sys.argv)
    next(ITERARGS) # skip first argument
    for s in ITERARGS:
        print(str(s))

# loop through argument list
ITERARGS2 = iter(sys.argv)
next(ITERARGS2) # skip first
for t in ITERARGS2: # python files cannot have any indentation
    infile = open(str(t), "r")
    for line in infile:
        node = ast.parse(infile.read())
        f = ast.fix_missing_locations(node)
        f = ast.dump(f, False, True)
        print(f)



################
# get the file #
################

# parse command line arguments to figure out which files should be parsed


########################################
# read the code from the file into ast #
########################################
# tree = ast.parse("print('hello world')")

#########################
# convert ast into json #
#########################

#################
# write to file #
#################

#######################
# send through socket #
#######################