#![no_std]
#![no_main]

extern crate alloc;
extern crate riscv_rt;
extern crate embedded_alloc;

use alloc::vec::Vec;

use embedded_alloc::Heap;

use riscv_rt::entry;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}


#[export_name = "DefaultHandler"]
fn dead_loop_int_handler() -> ! {
    loop {}
}

#[export_name = "ExceptionHandler"]
fn dead_loop_except_handler() -> ! {
    loop {}
}

#[export_name = "__pre_init"]
fn pre_init_cpu() {
    ()
}

#[export_name = "_setup_interrupts"]
fn setup_interrupts() {
    ()
}

#[cfg(feature = "llff")]
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut xs = Vec::new();
    xs.push(1);

    #[allow(clippy::empty_loop)]
    loop { /* .. */ }
}

