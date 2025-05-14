#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn prefetch(addr: *const u8) {
    core::arch::x86_64::_mm_prefetch(addr as *const i8, core::arch::x86_64::_MM_HINT_T0);
}
