#![no_std]
#![no_main]

mod lang_items;

// fn main() {
//     println!("Hello, world!");
// }
//
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
