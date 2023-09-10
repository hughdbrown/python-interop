#/usr/bin/env python3
"""Simple interop test"""
import sys

def fib(n):
    """Yet another fibonacci example"""
    # Does fibonacci sequence start at (0, 1, 1, ...) or (1, 1, 2, ...)?
    if n in (0, 1):
        return n
    return fib(n - 1) + fib(n - 2)


def test_10():
    """Driver"""
    for num in range(10):
        f_num = fib(num)
        print(f"{num}: {f_num}")

def main(num):
    """Driver"""
    f_num = fib(num)
    print(f"{num}: {f_num}")


if __name__ == '__main__':
    main(int(sys.argv[1]))
