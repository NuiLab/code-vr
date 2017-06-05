'''
simple exercise in abstract syntax trees
'''
import ast

NODE = ast.Expression(ast.BinOp(ast.Str('xy'), ast.Mult(), ast.Num(2)))
FIXED = ast.fix_missing_locations(NODE)

CODEOBJ = compile(FIXED, 'string', 'eval')
print(eval(CODEOBJ))