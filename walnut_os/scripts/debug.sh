#!/bin/bash

cargo run -- -s -S &
gdb ./target/x86_64-walnut/debug/walnut_os -ex "target remote localhost:1234"
