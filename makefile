#####
## BUILD
#####
#CC=riscv64-unknown-linux-gnu-g++
CC=/opt/riscv-walnut/bin/riscv64-unknown-linux-gnu-g++
CFLAGS=-Wall -Wextra -pedantic -Wextra -O0 -g -std=c++17
CFLAGS+=-static -ffreestanding -nostdlib -fno-rtti -fno-exceptions
CFLAGS+=-march=rv64gc -mabi=lp64d 
INCLUDES=
LINKER_SCRIPT=-Tsrc/lds/virt.lds
TYPE=debug
RUST_TARGET=./target/riscv64gc-unknown-none-elf/$(TYPE)
LIBS=-L$(RUST_TARGET)
SOURCES_ASM=$(wildcard src/asm/*.s)
LIB=-lwalnut -lgcc
OUT=./target/kernel/debug/os.elf

#####
## QEMU
#####
QEMU=qemu-system-riscv64
MACH=virt
CPU=rv64
CPUS=4
MEM=128M
DRIVE=./target/hdd.dsk

all:
	cargo build
	$(CC) $(CFLAGS) $(LINKER_SCRIPT) $(INCLUDES) -o $(OUT) $(SOURCES_ASM) $(LIBS) $(LIB)
	
run: all
	$(QEMU) -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM)   -serial mon:stdio -bios none -kernel $(OUT) -drive if=none,format=raw,file=$(DRIVE),id=foo -device virtio-blk-device,scsi=off,drive=foo

disk:
	dd if=/dev/zero of=$(DRIVE) bs=1M count=32

debug: all
	qemu-system-riscv64 -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM) -serial mon:stdio -bios none -kernel $(OUT) -drive if=none,format=raw,file=$(DRIVE),id=foo -device virtio-blk-device,scsi=off,drive=foo -s -S &
	/opt/riscv-walnut/bin/riscv64-unknown-linux-gnu-gdb --tui $(OUT) -ex "target remote :1234"

.PHONY: clean
clean:
	cargo clean
	rm -f $(OUT)
