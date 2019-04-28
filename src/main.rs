#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]

mod vga_buffer;

//static HELLO: &[u8] = b"Hello bob!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
//    let vga_buffer = 0xb8000 as *mut u8;
//
//    for (i, &byte) in HELLO.iter().enumerate() {
//        unsafe {
//            *vga_buffer.offset(i as isize * 2) = byte;
//            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
//        }
//    }

    vga_buffer::print_to_screen();


    loop {};
}

#[panic_handler]
fn bob_panic(_info: &core::panic::PanicInfo) -> ! {
    //hang the program on panic
    loop {};
}