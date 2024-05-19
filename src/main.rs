#![no_std]
#![no_main]
mod uefi;
use uefi::*;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "efiapi" fn efi_main(
    handle: ImageHandle,
    system_table: *const SystemTable
) {
    let string = "hello\n\r";
    for character in string.chars(){
        let mut buffer:[u16;1] = [0];
        let utf16 = character.encode_utf16(&mut buffer);
        unsafe{
            let status = ((*(*system_table).output).output_string)(
                (*system_table).output, 
                &utf16[0],
                );
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}