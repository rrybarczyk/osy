/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    check_alignment(align);
    addr & !(align - 1)
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    check_alignment(align);
    align_down(addr.saturating_add(align - 1), align)
}

fn check_alignment(align: usize) {
    if align == 0 || !align.is_power_of_two() {
        panic!("alignment must be greater than 0 and a power of 2");
    }
}
