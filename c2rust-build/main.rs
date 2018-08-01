#![feature(libc)]
#![feature(const_ptr_null)]
#![feature(offset_to)]
#![feature(const_ptr_null_mut)]
#![feature(extern_types)]
#![feature(asm)]
#![feature(main)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]


extern crate libc;

#[path = "../buffer.rs"] pub mod buffer;
#[path = "../test.rs"] pub mod test;
fn main() -> () { test::main() }
