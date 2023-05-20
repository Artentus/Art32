#!/bin/bash

find ".." -maxdepth 1 -iname "*.qrz" -print0 | xargs -0 ./quartz --top Art32 --output ./obj/art32.sv
yosys -l ./log/yosys.log -p "read_verilog -sv ./obj/art32.sv; read_verilog ../kernel_ram.v ../pll.v ../mult16.v ../uart.v ../ddr.v ../tmds_encoder.v; synth_ecp5 -top Art32 -json ./obj/art32.json; write_verilog ./obj/art32.v"
nextpnr-ecp5 -l ./log/nextpnr.log -r --25k --package CABGA256 --freq 40 --json ./obj/art32.json --lpf ./pins.lpf --lpf-allow-unconstrained --textcfg ./obj/art32.cfg
ecppack ./obj/art32.cfg ./bin/art32.bin
