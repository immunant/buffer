#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(linkage)]
#![feature(ptr_wrapping_offset_from)]
#![feature(new_uninit)]
#![feature(assoc_char_funcs)]

extern crate libc;
extern crate byte_strings;

pub mod util;
pub mod buffer;
