# Compiler and linker
CC = /opt/riscv-walnut/bin/riscv64-unknown-elf-gcc
LD = /opt/riscv-walnut/bin/riscv64-unknown-elf-ld
AS = /opt/riscv-walnut/bin/riscv64-unknown-elf-as

BUILD_DIR=build
INCLUDES := -I./src/kernel/include

FEATURE_FLAGS := -DUART_ENABLED=1

CFLAGS := -ffreestanding -nostartfiles -nostdlib -nodefaultlibs -g -Wl,--gc-sections -mcmodel=medany -march=rv64g -Wl,--no-warn-rwx-segments -mabi=lp64d -Wall -Wextra -O0 -g $(INCLUDES) $(FEATURE_FLAGS)
ASFLAGS= -g



# Source Files
OS_SOURCES := $(shell find src/ -name '*.c')
OS_ASM_SOURCES := $(shell find src/ -name '*.s')


# Object files
OS_OBJECTS := $(OS_SOURCES:.c=.o) $(OS_ASM_SOURCES:.s=.o)
OS_OBJECTS := $(addprefix $(BUILD_DIR)/, $(OS_OBJECTS))

# Name of your kernel executable
KERNEL_TARGET = $(BUILD_DIR)/walnut

# Linking step 
$(BUILD_DIR)/$(KERNEL_TARGET): $(OS_OBJECTS)
	$(LD) -T linker.ld -o $(KERNEL_TARGET) $(OS_OBJECTS)

# Compilation rules
$(BUILD_DIR)/%.o: %.c
	$(MKDIR_P) $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.s
	$(MKDIR_P) $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

all: build


# Build target
build: $(BUILD_DIR)/$(KERNEL_TARGET)

# Run target
run: build
	qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 512M -serial mon:stdio -bios none -kernel $(KERNEL_TARGET) 

debug: build
	qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 512M -serial mon:stdio -bios none -kernel $(KERNEL_TARGET) -s -S > /dev/null 2>&1 &
	riscv64-unknown-elf-gdb $(KERNEL_TARGET) --tui -ex "target remote :1234" -x ./gdb/config.gdb

sd: build
	qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 512M -serial mon:stdio -bios none -kernel $(KERNEL_TARGET) -s -S &
	QT_AUTO_SCREEN_SET_FACTOR=0 QT_SCALE_FACTOR=2 QT_FONT_DPI=96 seer --connect localhost:1234 --sat --cwd ~/Source/walnut/ $(KERNEL_TARGET)

no_display_run: build
	qemu-system-riscv64 -machine virt -nographic -bios none -kernel $(KERNEL_TARGET) 


# Clean target 
clean:
	rm -f $(KERNEL_TARGET) $(OS_OBJECTS)

MKDIR_P = mkdir -p
