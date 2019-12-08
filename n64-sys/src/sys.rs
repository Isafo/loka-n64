use core::ffi::c_void;

#[inline]
pub(crate) unsafe fn data_cache_hit_writeback_invalidate(block: &mut [u64]) {
    let mut addr = ((block.as_mut_ptr() as usize) & 0xffff_fffc) as *mut c_void;
    let mut len = block.len() * 8;

    while len > 0 {
        asm!("cache $0, ($1)"
        :
        : "i" (0x15), "r" (addr)
        :
        : "volatile"
        );

        len -= 4;
        addr = addr.offset(4);
    }
}

#[inline]
pub(crate) unsafe fn uncached_addr<T>(address: *const T) -> *const T {
    ((address as usize) | 0x2000_0000) as *const T
}

#[inline]
pub(crate) unsafe fn uncached_addr_mut<T>(address: *mut T) -> *mut T {
    ((address as usize) | 0x2000_0000) as *mut T
}

#[inline]
pub(crate) unsafe fn virtual_to_physical<T>(address: *const T) -> usize {
    (address as usize) & 0x1fffffff
}

#[inline]
pub(crate) unsafe fn virtual_to_physical_mut<T>(address: *mut T) -> usize {
    (address as usize) & 0x1fffffff
}

#[inline]
pub(crate) unsafe fn enable_interrupts() {
    asm!("mfc0 $$8,$$12
        ori $$8,1
        mtc0 $$8,$$12
        nop"
        ::: "$$8");
}

#[inline]
pub(crate) unsafe fn disable_interrupts() {
    asm!("mfc0 $$8,$$12
        la $$9,~1
        and $$8,$$9
        mtc0 $$8,$$12
        nop"
        ::: "$$8", "$$9");
}

#[inline]
pub(crate) unsafe fn memory_barrier() {
    asm!("" ::: "memory" : "volatile");
}

#[inline]
fn get_tick_rate() -> u32 {
    93750000 / 2
}

#[inline]
fn get_ticks() -> u32 {
    let res;

    unsafe {
        asm!("mfc0 $0,$$9
            nop"
            : "=r" (res));
    }

    res
}

#[inline]
pub fn get_ticks_us() -> i32 {
    (get_ticks() / (get_tick_rate() / (1000 * 1000))) as i32
}