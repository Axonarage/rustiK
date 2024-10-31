//! Initialization of Cortex-M device
//! 
//! Cortex-M devices are using a vector table for initialization.
//! It is located at the start of the code region in memory.
//! 
//! The first 2 elements of the vector table are :
//!   - the **initial Stack Pointer**
//!   - the **Reset vector** (pointer to the Reset handler)
//! 
//! This crates will populates the vector table.

#![no_std]
#![no_main]

use core::arch::asm;

mod panic;

// Here is the Reset vector, a pointer to our Reset handler : Reset()
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

// Our Reset handler, wich isn't doing much for now
#[unsafe(no_mangle)]
pub extern "C" fn Reset() -> ! {
    
    unsafe {
        asm!(
            "mov r0, #100"
        );
    }
    
    loop {}
}