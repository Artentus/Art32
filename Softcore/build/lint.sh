#!/bin/bash

find ".." -maxdepth 1 -iname "*.qrz" -print0 | xargs -0 ./quartz --top Art32 --output ./obj/art32.sv
find ".." -maxdepth 1 -iname "*.v" -print0 | xargs -0 verilator --lint-only -Wall -Wno-DECLFILENAME -Wno-UNUSED -Wno-UNSIGNED -Wno-SYMRSVDWORD -DSIM --top-module Art32 +1800-2005ext+sv ./obj/art32.sv +1364-2005ext+v
