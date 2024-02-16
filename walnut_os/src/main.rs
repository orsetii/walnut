#![no_std]
#![no_main]
#![feature(custom_test_frameworks, const_mut_refs, const_ptr_as_ref, const_option)]

use bootloader::{entry_point, BootInfo};
use walnut_os::{arch::amd64::memory::translate_addr, println, util::hlt_loop};
use x86_64::VirtAddr;

entry_point!(kmain);

#[no_mangle]
fn kmain(info: &'static BootInfo) -> ! {
    walnut_os::println!("{:#?}", info);
    walnut_os::println!("Walnut OS booting...");
    walnut_os::arch::initialize();

    let phys_mem_offset = VirtAddr::new(info.physical_memory_offset);
    let phys_mem_offset = VirtAddr::new(info.physical_memory_offset);

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

    hlt_loop();
}
