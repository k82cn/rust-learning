
import flame
import random
import math

@flame.service
def add(x: int, y: int) -> int:
    return x + y

@flame.service
def pi(n):
    sum = 0.0

    for i in range(n):
        x, y = random.uniform(0, 1.0), random.uniform(0, 1.0)
        if math.hypot(x, y) <= 1.0:
            sum += 1

    return sum

# print(add(4, y=5))

print(pi(10))