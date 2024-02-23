#!/bin/bash
#

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

function get_random_port() {
  while true; do
    local PORT=$((RANDOM % (65535 - 1024) + 1024))
    if ! ss -lnt | grep -q ":$PORT "; then
      echo "$PORT"
      break
    fi
  done
}

if [[ ${@: -1} == "debug" ]]; then  # Prioritize "debug" in the last argument
    MODE="debug" 
    echo "Explicit 'debug' specified. Entering debug mode"
elif [[ ${@: -1} == *"target"* ]]; then  
    MODE="run"  
    echo "'target' detected in arguments. Entering run mode"
elif [[ ${@: -1} == "disas" ]]; then
    # Get the script's directory and execute disas.sh
    "$SCRIPT_DIR/disas.sh" "$@"  # Pass any arguments to disas.sh
    exit 0
else  
    # Check if enough arguments were provided
    if [[ $# -lt 1 ]]; then 
        echo "No mode specified. Please provide 'run' or 'debug'"
        exit 1
    fi

    MODE="${@: -1}"  # Extract the last argument as the mode
    shift $(($# - 1))  # Shift arguments to remove the mode
fi



# Build common QEMU command components
QEMU_BASE="qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 128M -nographic -serial mon:stdio -bios none -device virtio-keyboard-device -kernel"

EXPECTED_TARGET_PATH=$SCRIPT_DIR/../../target/riscv64gc-unknown-none-elf/debug/walnut

if [[ $MODE == "run" ]]; then
    COMMAND="$QEMU_BASE $@"  # Pass additional arguments for run mode
elif [[ $MODE == "debug" ]]; then
    GDB_PORT=$(get_random_port)

    COMMAND="$QEMU_BASE $EXPECTED_TARGET_PATH -gdb tcp::$GDB_PORT -S "

    echo "Executing: $COMMAND" 

    $COMMAND &

    konsole --new-tab -e /opt/riscv/bin/riscv64-unknown-elf-gdb $EXPECTED_TARGET_PATH -ex "target remote localhost:$GDB_PORT" -x "$SCRIPT_DIR/config.gdb"


    exit 0
else
    echo "Invalid mode. Choose 'run' or 'debug'"
    exit 1
fi

# Execute the command
echo "Executing: $COMMAND" 
$COMMAND
