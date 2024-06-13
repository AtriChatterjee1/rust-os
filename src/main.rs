

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    // lock basically lock's the thread for this process, and blocks all other processes
    vga_buffer::WRITER.lock().write_str("HELLO WORLD").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers are {} and {}", 28 ,06).unwrap();
    println!();

    println!("Hello World{}", "!");
    panic!("Some panic message");
}

mod vga_buffer;



