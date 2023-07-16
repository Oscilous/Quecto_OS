#![no_std]
#![no_main]

mod pixel_buffer;

use core::panic::PanicInfo;

const MULTIBOOT2_BOOTLOADER_MAGIC: u64 = 0x36d76289;
#[repr(C)]
struct MultibootTag {
    typ: u32,
    size: u32,
}

#[repr(C)]
struct MultibootTagFramebuffer {
    typ: u32,
    size: u32,
    framebuffer_addr: u64,
    framebuffer_pitch: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u8,
    framebuffer_type: u8,
    reserved: u8,
}

static mut MAGIC_NUMBER: u64 = 0;
static mut INFOSTRUCT_ADDRESS: u64 = 0;

#[no_mangle]
pub extern "C" fn _start(magic: u64, addr: u64) -> ! {
    let magic_number: u64;
    let infostruct_address: u64;

    unsafe {
        magic_number = MAGIC_NUMBER;
        infostruct_address = INFOSTRUCT_ADDRESS;
    }

    if magic_number != MULTIBOOT2_BOOTLOADER_MAGIC {
        panic!();
    }
    let multiboot_addr = addr;
    unsafe {
        // Convert the Multiboot address to a raw pointer
        let multiboot_ptr = multiboot_addr as *const u64;

        // Find the tag by iterating over the Multiboot tags
        let mut current_ptr = multiboot_ptr.offset(8); // Skip the initial magic number and size

        loop {
            let tag = &*(current_ptr as *const MultibootTag);

            if tag.typ == 0 {
                // End tag
                break;
            }

            match tag.typ {
                8 => {
                    let framebuffer_tag = &*(current_ptr as *const MultibootTagFramebuffer);
                    // Access the framebuffer_tag fields here
                    // framebuffer_tag.framebuffer_addr, framebuffer_tag.framebuffer_pitch, etc.
                    // Handle the framebuffer data as needed
                }
                _ => {
                    // Handle other tag types if necessary
                }
            }

            // Move to the next tag
            current_ptr = current_ptr.offset(((tag.size + 7) & !7).try_into().unwrap());
            // Align to 8-byte boundary
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
