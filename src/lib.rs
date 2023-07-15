#![no_std]
#![no_main]

mod pixel_buffer;

use core::panic::PanicInfo;
/*
struct multiboot_tag_framebuffer {
    uint64_t address;
    uint32_t pitch;
    uint32_t width;
    uint32_t height;
    uint8_t bpp;
    uint8_t type;
    // Technically, the layout of the following fields depends on the value of `type`
    uint8_t red_position, red_mask_size;
    uint8_t green_position, green_mask_size;
    uint8_t blue_position, blue_mask_size;
} __attribute__ ((packed)) fb_info_t;
*/

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

#[no_mangle]
pub extern "C" fn _start(magic: u64, addr: u64) -> ! {
    /*
    let vga_buffer = 0xb8000 as *mut u8;

    let magic: u64 = 111111;

    let bytes = magic.to_le_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    */
    let multiboot_magic = magic;
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
