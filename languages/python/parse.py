import ast


def parse(source):
    assert(isinstance(source, str))
    tree = ast.parse(source)
    return tree