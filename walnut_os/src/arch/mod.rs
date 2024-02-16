#[cfg(target_arch = "x86_64")]
pub mod amd64;

pub fn initialize() {
    #[cfg(target_arch = "x86_64")]
    amd64::initialize();
}
