use crate::io::aarch64;

pub fn time_sec_frac() -> (u64, u32) {
    let v = aarch64::cntvct_el0();
    let f = aarch64::cntfrq_el0();

    let secs = v / f;
    let rem = v % f;

    let micros = (rem * 1_000_000) / f;

    (secs, micros as u32)
}
