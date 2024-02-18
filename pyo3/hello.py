
import flame

@flame.service
def add(x: int, y: int) -> int:
    return x + y

print(add(4, y=5))
