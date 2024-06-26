pub const KERNEL_STACK_SIZE: usize = 4096 * 2; // 8KiB
pub const USER_STACK_SIZE: usize = 4096 * 2; // 8KiB

// qemu-system-riscv64 -machine virt,dumpdtb=dump.dtb
// dtc dump.dtb | vi -
pub const TIMEBASE_FREQUENCY: usize = 0x989680;
pub const TICK_PER_SEC: usize = 100;
pub const MSEC_PER_SEC: usize = 1000;

pub const HEAP_ORDER_SIZE: usize = 32;
pub const KERNEL_HEAP_SIZE: usize = 0x3_00000;

// page
pub const PAGE_SIZE_BITS: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS;
pub const SV39_ADDR_WIDTH: usize = 56;
pub const SV39_PPN_WIDTH: usize = SV39_ADDR_WIDTH - PAGE_SIZE_BITS;
// start of the last page of the usize
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

pub const MEMORY_END: usize = 0x8800_0000;

pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}
