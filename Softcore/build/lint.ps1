$qrzFiles = (Get-ChildItem -Path ".." -Filter "*.qrz" -File).FullName
./quartz.exe --top Art32 --output ./obj/art32.sv $qrzFiles

$vFiles = (Get-ChildItem -Path ".." -Filter "*.v" -File).FullName.replace('\', '/')
verilator_bin.exe --lint-only -Wall -Wno-DECLFILENAME -Wno-UNUSED -Wno-UNSIGNED -Wno-SYMRSVDWORD -DSIM --top-module Art32 +1800-2005ext+sv ./obj/art32.sv +1364-2005ext+v $vFiles
