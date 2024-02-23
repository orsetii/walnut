#!/bin/bash


cargo build --quiet
riscv64-unknown-elf-objdump -d target/riscv64gc-unknown-none-elf/debug/walnut
