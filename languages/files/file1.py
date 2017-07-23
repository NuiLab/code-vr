orderedPairsSet = set()

def sum(x,y):
    return x + y

def div(x,y):
    if (x % y == 0):
        return True
    else:
        return False

def checkPair(x,y,p,q):
    if(sum(x,y) > p and sum(x,y) < q and div(y,x)==True):
        orderedPairsSet.add((x,y))
        return True
    else: 
        return False

def myRelation(p,q):
    if(p < q and p > 3 and q > 4):
        print("myRelation(%d, %d)" % (p,q))
        x = 1
        y = 2

        checkPair(x,y,p,q)
        while (x <= p and y <= q):
            if (y+2 > q):
                y = 2
                x = x+2
                checkPair(x,y,p,q)
            else:
                y = y+2
                checkPair(x,y,p,q)
                
        print("List:")
        print(sorted(orderedPairsSet))

    else:
        print("invalid input")

myRelation(5,10)
myRelation(6,15)