import random
import string

def greet(name):
    return f"Hello, {name}!"

def add(a, b):
    return str(int(a) + int(b))

def generate_random_string(length=10):
    """
    生成的随机字符串。
    """
    characters = string.ascii_letters + string.digits
    random_string = ''.join(random.choice(characters) for i in range(length))
    return random_string