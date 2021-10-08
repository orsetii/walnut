//! Boot code.

#[cfg(target_arch = "aarch64")]
#[path = "../arch/aarch64/cpu/boot.rs"]
mod arch_boot;
