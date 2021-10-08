#!/bin/bash


BIN_PATH=${CARGO_MAKE_WORKING_DIRECTORY}/target/aarch64-unknown-none-softfloat/debug/walnut

echo "Running QEMU..."

# Preserve exit code before we pipe to sed 
# Then remove VT100 Escape Codes
# And remove QEMU UEFI output
set -o pipefail
qemu-system-aarch64 \
  -M raspi3 \
  -serial stdio \
  -smp 4 \
  -display none  \
  -kernel $BIN_PATH


# Check for the desired QEMU exit code, and exit with 0
# Else, exit with the original code
[ $? -eq 33 ] && exit 0 || exit $QEMU_EXITCODE 
