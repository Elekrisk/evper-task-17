#!/usr/bin/python3
import sys;
import time;

x = 0
s = time.time()
for line in sys.stdin:
    x += 1
e = time.time()
print(x)
print(e - s)