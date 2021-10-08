#!/bin/bash

DEFAULT_BIN_PATH="target/kernel"

BASE_DIR="$(dirname $0)/../"

if [ $# -lt 2 ]; then
  if [ ! -f DEFAULT_BIN_PATH ]; then
    echo "Please provide a binary name as an argument and an architecture"
    exit 1
  else
    BIN_PATH=$DEFAULT_BIN_PATH
  fi
else
  BIN_PATH=$1
fi

if [ "$1" = "" ]; then
  BIN_PATH=$DEFAULT_BIN_PATH
fi

mkdir -p $BASE_DIR/target/EFI/BOOT
cp $BIN_PATH $BASE_DIR/target/EFI/BOOT/BOOTx64.EFI

ARCH=$2

echo "Running QEMU..."

# Preserve exit code before we pipe to sed 
# Then remove VT100 Escape Codes
# And remove QEMU UEFI output
set -o pipefail
qemu-system-$(ARCH) \
  -serial stdio \
  -smp 4 \
  -nographic | \
  -kernel $BIN_PATH \
  grep -v "BdsDxe" # Removes useless QEMU UEFI output and strips VT100 colour codes


# Check for the desired QEMU exit code, and exit with 0
# Else, exit with the original code
[ $? -eq 33 ] && exit 0 || exit $QEMU_EXITCODE 
