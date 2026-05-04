//! Power State Coordination Interface calls exposed by QEMU's `virt` machine.
const PSCI_SYSTEM_OFF: u64 = 0x8400_0008;

pub fn system_off() -> ! {
    unsafe {
        core::arch::asm!(
            "hvc #0",
            in("x0") PSCI_SYSTEM_OFF,
            options(nostack)
        );
    }

    loop {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack));
        }
    }
}
