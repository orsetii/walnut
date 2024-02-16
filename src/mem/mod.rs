use crate::*;

pub mod kalloc;
pub mod page;

pub fn init() -> usize {
    // The job of mem::init() is to get us into supervisor mode
    // as soon as possible.
    // Interrupts are disabled for this kinit()

    page::init();
    kalloc::init();

    // Map heap allocations
    let root_ptr = kalloc::get_page_table();
    let root_u = root_ptr as usize;
    let mut root = unsafe { root_ptr.as_mut().unwrap() };
    let kheap_head = kalloc::get_head() as usize;
    let total_pages = kalloc::get_num_allocations();
    println!();
    println!();
    unsafe {
        println!("TEXT:   0x{:x} -> 0x{:x}", TEXT_START, TEXT_END);
        println!("RODATA: 0x{:x} -> 0x{:x}", RODATA_START, RODATA_END);
        println!("DATA:   0x{:x} -> 0x{:x}", DATA_START, DATA_END);
        println!("BSS:    0x{:x} -> 0x{:x}", BSS_START, BSS_END);
        println!(
            "STACK:  0x{:x} -> 0x{:x}",
            KERNEL_STACK_START, KERNEL_STACK_END
        );
        println!(
            "HEAP:   0x{:x} -> 0x{:x}",
            kheap_head,
            kheap_head + total_pages * 4096
        );
    }
    id_map_range(
        &mut root,
        kheap_head,
        kheap_head + total_pages * 4096,
        EntryBits::ReadWrite.val(),
    );
    unsafe {
        // Map heap descriptors
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        id_map_range(
            &mut root,
            HEAP_START,
            HEAP_START + num_pages,
            EntryBits::ReadWrite.val(),
        );
        // Map executable section
        id_map_range(
            &mut root,
            TEXT_START,
            TEXT_END,
            EntryBits::ReadExecute.val(),
        );
        // Map rodata section
        // We put the ROdata section into the text section, so they can
        // potentially overlap however, we only care that it's read
        // only.
        id_map_range(
            &mut root,
            RODATA_START,
            RODATA_END,
            EntryBits::ReadExecute.val(),
        );
        // Map data section
        id_map_range(&mut root, DATA_START, DATA_END, EntryBits::ReadWrite.val());
        // Map bss section
        id_map_range(&mut root, BSS_START, BSS_END, EntryBits::ReadWrite.val());
        // Map kernel stack
        id_map_range(
            &mut root,
            KERNEL_STACK_START,
            KERNEL_STACK_END,
            EntryBits::ReadWrite.val(),
        );
    }

    // UART
    map(
        &mut root,
        0x1000_0000,
        0x1000_0000,
        EntryBits::ReadWrite.val(),
        0,
    );

    // CLINT
    //  -> MSIP
    map(
        &mut root,
        0x0200_0000,
        0x0200_0000,
        EntryBits::ReadWrite.val(),
        0,
    );
    //  -> MTIMECMP
    map(
        &mut root,
        0x0200_b000,
        0x0200_b000,
        EntryBits::ReadWrite.val(),
        0,
    );
    //  -> MTIME
    map(
        &mut root,
        0x0200_c000,
        0x0200_c000,
        EntryBits::ReadWrite.val(),
        0,
    );
    // PLIC
    id_map_range(
        &mut root,
        0x0c00_0000,
        0x0c00_2000,
        EntryBits::ReadWrite.val(),
    );
    id_map_range(
        &mut root,
        0x0c20_0000,
        0x0c20_8000,
        EntryBits::ReadWrite.val(),
    );
    print_page_allocations();
    // The following shows how we're going to walk to translate a virtual
    // address into a physical address. We will use this whenever a user
    // space application requires services. Since the user space application
    // only knows virtual addresses, we have to translate silently behind
    // the scenes.
    let p = 0x8005_7000 as usize;
    let m = virt_to_phys(&root, p).unwrap_or(0);
    println!("Walk 0x{:x} = 0x{:x}", p, m);
    // When we return from here, we'll go back to boot.S and switch into
    // supervisor mode We will return the SATP register to be written when
    // we return. root_u is the root page table's address. When stored into
    // the SATP register, this is divided by 4 KiB (right shift by 12 bits).
    // We enable the MMU by setting mode 8. Bits 63, 62, 61, 60 determine
    // the mode.
    // 0 = Bare (no translation)
    // 8 = Sv39
    // 9 = Sv48
    unsafe {
        // We have to store the kernel's table. The tables will be moved back
        // and forth between the kernel's table and user applicatons' tables.
        KERNEL_TABLE = root_u;
    }
    // table / 4096    Sv39 mode
    (root_u >> 12) | (8 << 60)
}
