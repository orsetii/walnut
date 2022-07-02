

echo "Building Walnut..."

(cd bootloader && cargo build)
echo "Bootloader complete..."

(cd kernel && cargo build)

echo "Kernel complete..."

echo "Fully Built. Running Walnut via QEMU..."

mkdir -p build/vmroot/EFI/{kernel,Boot}

cp ./target/amd64/debug/kernel build/vmroot/EFI/kernel/kernel.elf
cp ./target/x86_64-unknown-uefi/debug/bootloader.efi build/vmroot/EFI/Boot/BootX64.efi





qemu-system-x86_64 \
    -bios /usr/share/ovmf/OVMF.fd \
    -drive format=raw,file=fat:rw:build/vmroot \
	-device ahci,id=ahci0 \
	-device isa-debug-exit \
    -m 4096 \
    -smp 2 \
    -serial mon:stdio \
    -nographic \
    -s


