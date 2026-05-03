//! wrapping aarch64 system register accesses

/// codegen for wrapping fetches of aarch64 system registers
macro_rules! sysreg {
    ($(#[$meta:meta])* $r:ident) => {
        #[doc = concat!("fetches the raw value of the ", stringify!($r), " aarch64 register")]
        $(#[$meta])*
        #[inline(always)]
        pub fn $r() -> u64 {
            let val: u64;
            unsafe {
                core::arch::asm!(concat!("mrs {}, ", stringify!($r)), out(reg) val);
            }
            val
        }
    };
}

sysreg! {
    /// see: https://developer.arm.com/documentation/ddi0601/2021-12/AArch64-Registers/CNTVCT-EL0--Counter-timer-Virtual-Count-register
    cntvct_el0
}
sysreg! {
    /// see: https://developer.arm.com/documentation/ddi0601/2020-12/AArch64-Registers/CNTFRQ-EL0--Counter-timer-Frequency-register
    cntfrq_el0
}
