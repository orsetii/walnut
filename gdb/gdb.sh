#!/bin/bash
#
DIR=$(dirname "$(readlink -f "$0")")


riscv64-unknown-elf-gdb ./target/riscv64gc-unknown-none-elf/debug/macaque --tui -ex "target remote :1234" -x $DIR/config.gdb

