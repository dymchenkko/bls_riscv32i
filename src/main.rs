#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
use signature_bls::{Signature, PublicKey};
extern crate alloc;
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::arch::asm;
use data_encoding::HEXLOWER;
extern crate panic_halt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
        let message = "080907000102040506070809020c0304080907000102040506070809020c0304";
        let signat = "8613038c1684b048b63328b35d31fe14a144010316fe25bde58a268674ab3ae5bfdb910c9fa7c347e4c3d4063d305c8c";
        let pk = "a2359fa8a69cc2bc8c71017402fc9faf43159354173c72822c0b4685c2c0acd4cfb06349dc883c7c23150f93eeb390780b486303eea37af882f62cb3ab5cecd03dffbbb8a11f99950007764da9c730d2e1604b4f7e4a1c570aa88f84e3bd22c1";
    
        let result = Signature::from_bytes(&HEXLOWER.decode(&signat.as_bytes()).unwrap().try_into().unwrap()).unwrap().verify(PublicKey::from_bytes(&HEXLOWER.decode(&pk.as_bytes()).unwrap().try_into().unwrap()).unwrap(), &HEXLOWER.decode(message.as_bytes()).unwrap());
        assert_eq!(result.unwrap_u8(), 1);  
        loop {
            delay(300000);
        }  
}

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            asm!("nop");
        }
    }
}