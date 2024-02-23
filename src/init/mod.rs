//! Initialization routines and structures to set up the operating system
//! this contains the first rust code that is executed by Walnut.

use crate::{
    cpu::{
        mode::{set_prev_privilege_mode, Mode},
        util::my_hart,
    },
    println,
};

#[no_mangle]
extern "C" fn kinit() {
    println!("Initializing Hardware Thread {}", my_hart());
    set_prev_privilege_mode(Mode::Supervisor);

    panic!();
}
