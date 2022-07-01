use core::alloc::Layout;
use core::panic::PanicInfo;
use crate::{debug, print, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();
    let message = info.message().unwrap();
    print!("\n");
    print!(31;r"
IIIIIIII           III            III     II     OO     IIIIIIIIIIII
II     II         II  II          IIII    II            II
II     II        II    II         II II   II     II     II
II     II       IIIIIIIIII        II  II  II     II     II
IIIIIIII       II        II       II   II II     II     II
II            II          II      II    IIII     II     II
II           II            II     II     III     II     IIIIIIIIIIIII
");
    print!("\n");
    debug!(
        "\nPANIC in {} at line {} \n\t{}",
        location.file(),
        location.line(),
        message
    );
    loop {}
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort!");
}

#[lang = "oom"]
fn oom(layout: Layout) -> ! {
    panic!("Memory allocation of {} bytes failed", layout.size());
}

#[lang = "eh_personality"]
#[no_mangle]
fn eh_personality() -> ! {
    loop {}
}
