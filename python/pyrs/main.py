import sys
from . import sum_as_string


def cli():
    print(sum_as_string(int(sys.argv[1]), int(sys.argv[2])))
