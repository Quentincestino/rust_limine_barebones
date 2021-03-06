#![no_std]
#![no_main]
#![feature(llvm_asm)]

use core::panic::PanicInfo;
use stivale_boot::v2::{StivaleFramebufferHeaderTag, StivaleHeader};

static STACK: [u8; 4096] = [0; 4096];

static FRAMEBUFFER_TAG: StivaleFramebufferHeaderTag =
    StivaleFramebufferHeaderTag::new().framebuffer_bpp(24);

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(&STACK[4095] as *const u8)
    .tags((&FRAMEBUFFER_TAG as *const StivaleFramebufferHeaderTag).cast());

#[no_mangle]
extern "C" fn entry_point(_header_addr: usize) -> ! {
    for &char in b"Hello, World !".iter() {
        unsafe {
            let port = 0x3F8; // COM1
            llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(char));
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_infos: &PanicInfo) -> ! {
    loop {}
}
