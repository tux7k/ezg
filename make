#!/bin/bash
cargo run
gnuplot -p -e "cd 'C:\Users\tux\Desktop\ezg\'; plot for [col=2:3] 'out.log' using 0:col with linespoints"