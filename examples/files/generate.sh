#!/bin/sh

diff -u input.txt output1.txt > diff1.patch
diff -u input.txt output2.txt > diff2.patch
diff -u input.txt output3.txt > diff3.patch