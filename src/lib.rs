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

struct multiboot_tag {
    _type: u32,
    size: u32,
}

struct multiboot_info {
    total_size: u32,
    reserved: u32,
    tags: multiboot_tag,
}

#[no_mangle]
pub extern "C" fn _start(magic: u64, addr: u64) -> ! {
    //let pos_y =
    //let location: *const char = (unsigned *char)0xA0000 + 320 * pos_y + pos_x;
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
