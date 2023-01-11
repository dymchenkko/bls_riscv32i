#![feature(asm_experimental_arch)]
#![feature(alloc_error_handler)] // no_std and allocator support is not stable.
#![feature(stdsimd)] // for `mips::break_`. If desired, this could be replaced with asm.
#![no_std]
#![no_main]

pub extern crate externc_libm as libm;
use signature_bls::{SecretKey, Signature, PublicKey};
use data_encoding::HEXLOWER;

extern crate alloc;
use core::arch::asm;


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
           asm!("ebreak", options(noreturn));
    }
    // Uncomment code below if you're in trouble
    /* 
    let msg = alloc::format!("Panic: {}", info);
    iommu::print(&msg);
    */ 
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    // NOTE: avoid `panic!` here, technically, it might not be allowed to panic in an OOM situation.
    //       with panic=abort it should work, but it's no biggie use `break` here anyway.
    unsafe {
       asm!("ebreak", options(noreturn));
    }
}


fn exit() -> ! {
    unsafe {
        asm!(r#"lui	a0,0x0;
	ecall;"#, options(noreturn));
    }
}

use core::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
};

struct BumpPointerAlloc;


extern "C" {
        static mut __heap_start: usize;
}

#[no_mangle]
pub fn zkvm_abi_alloc_words(nwords: usize) -> *mut usize {
    let ptr = unsafe { (__heap_start as (*mut usize)).add(nwords * 8) };
    unsafe { __heap_start = ptr as usize; }
    ptr
}


unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let aligned_size = layout
            .align_to(8)
            .expect("Unable to align allocation to word size")
            .pad_to_align()
            .size() / 8;
        
        zkvm_abi_alloc_words(aligned_size) as *mut u8
    }
    
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.alloc(layout)
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // this allocator never deallocates memory
    }
}

#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc;



extern "C" {
    static mut _ram_start: usize;
}

#[no_mangle]
pub extern "C" fn __start() {
    unsafe { 
    let message = "080907000102040506070809020c0304080907000102040506070809020c0304";
    let signat = "8613038c1684b048b63328b35d31fe14a144010316fe25bde58a268674ab3ae5bfdb910c9fa7c347e4c3d4063d305c8c";
    let pk = "a2359fa8a69cc2bc8c71017402fc9faf43159354173c72822c0b4685c2c0acd4cfb06349dc883c7c23150f93eeb390780b486303eea37af882f62cb3ab5cecd03dffbbb8a11f99950007764da9c730d2e1604b4f7e4a1c570aa88f84e3bd22c1";

    let result = Signature::from_bytes(&HEXLOWER.decode(&signat.as_bytes()).unwrap().try_into().unwrap()).unwrap().verify(PublicKey::from_bytes(&HEXLOWER.decode(&pk.as_bytes()).unwrap().try_into().unwrap()).unwrap(), &HEXLOWER.decode(message.as_bytes()).unwrap());
    assert_eq!(result.unwrap_u8(), 1);    
    }
    exit();
}

core::arch::global_asm!(
    r#"
.section .text._start;
.globl _start;
_start:
    .option push;
    .option norelax;
    la gp, __global_pointer$;
    .option pop;
    la sp, __stack_init$;
    la a0, __input_begin
    jal ra, __start
"#
);