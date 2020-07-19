#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use libc::{size_t, ssize_t};
use buffer::c_slice;
use buffer::buffer::*;
use byte_strings::c_str;
extern "C" {
    #[no_mangle]
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}

//
// test.c
//
// Copyright (c) 2012 TJ Holowaychuk <tj@vision-media.ca>
//
#[no_mangle]
fn equal(a: *const libc::c_char, b: *const libc::c_char) {
    unsafe {
        if strcmp(a, b) != 0 {
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            printf(
                b"  expected: \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                a,
            );
            printf(
                b"    actual: \'%s\'\n\x00" as *const u8 as *const libc::c_char,
                b,
            );
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        };
    }
}

fn test_buffer_new() {
    let mut buf: buffer_t = buffer_new();
    assert!(64 == buffer_size(&buf));
    assert!(0 == buffer_length(&buf));
    buffer_free(buf);
}

fn test_buffer_new_with_size() {
    let mut buf = buffer_new_with_size(1024 as libc::c_int as size_t);
    assert!(1024 == buffer_size(&buf));
    assert!(0 == buffer_length(&buf));
    buffer_free(buf);
}

fn test_buffer_append() {
    let mut buf: buffer_t = buffer_new();
    assert!(0 == buffer_append(&mut buf, c_slice!(b"Hello")));
    assert!(0 == buffer_append(&mut buf, c_slice!(b" World")));
    assert!(
        buffer::util::strlen(c_slice!(b"Hello World"))
            == buffer_length(&buf)
    );
    equal(
        b"Hello World\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        unsafe { buf.data_ptr() },
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append_n() {
    let mut buf = buffer_new();
    assert!(0 == buffer_append_n(&mut buf, c_slice!(b"subway"), 3));
    assert!(0 == buffer_append_n(&mut buf, c_slice!(b"marines"), 6));
    assert!(buffer::util::strlen(c_slice!("submarine")) == buffer_length(&buf));
    equal(c_str!("submarine").as_ptr(), buf.data_ptr());
    buffer_free(buf);
}
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_append__grow() {
//     let mut buf: *mut buffer_t =
//         buffer_new_with_size(10 as libc::c_int as size_t);
//     if !(0 as libc::c_int ==
//              buffer_append(buf,
//                            b"Hello\x00" as *const u8 as *const libc::c_char))
//            as libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 25],
//                                                &[libc::c_char; 25]>(b"test_buffer_append__grow\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      64 as libc::c_int,
//                      b"0 == buffer_append(buf, \"Hello\")\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     if !(0 as libc::c_int ==
//              buffer_append(buf,
//                            b" tobi\x00" as *const u8 as *const libc::c_char))
//            as libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 25],
//                                                &[libc::c_char; 25]>(b"test_buffer_append__grow\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      65 as libc::c_int,
//                      b"0 == buffer_append(buf, \" tobi\")\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     if !(0 as libc::c_int ==
//              buffer_append(buf,
//                            b" was\x00" as *const u8 as *const libc::c_char))
//            as libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 25],
//                                                &[libc::c_char; 25]>(b"test_buffer_append__grow\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      66 as libc::c_int,
//                      b"0 == buffer_append(buf, \" was\")\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     if !(0 as libc::c_int ==
//              buffer_append(buf,
//                            b" here\x00" as *const u8 as *const libc::c_char))
//            as libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 25],
//                                                &[libc::c_char; 25]>(b"test_buffer_append__grow\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      67 as libc::c_int,
//                      b"0 == buffer_append(buf, \" here\")\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     let mut str: *mut libc::c_char =
//         b"Hello tobi was here\x00" as *const u8 as *const libc::c_char as
//             *mut libc::c_char;
//     equal(str, (*buf).data);
//     if !(1024 as libc::c_int as libc::c_ulong == buffer_size(buf)) as
//            libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 25],
//                                                &[libc::c_char; 25]>(b"test_buffer_append__grow\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      71 as libc::c_int,
//                      b"1024 == buffer_size(buf)\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     if !(strlen(str) == buffer_length(buf)) as libc::c_int as libc::c_long !=
//            0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 25],
//                                                &[libc::c_char; 25]>(b"test_buffer_append__grow\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      72 as libc::c_int,
//                      b"strlen(str) == buffer_length(buf)\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     buffer_free(buf);
// }
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_prepend() {
//     let mut buf: *mut buffer_t = buffer_new();
//     if !(0 as libc::c_int ==
//              buffer_append(buf,
//                            b" World\x00" as *const u8 as *const libc::c_char))
//            as libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 20],
//                                                &[libc::c_char; 20]>(b"test_buffer_prepend\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      79 as libc::c_int,
//                      b"0 == buffer_append(buf, \" World\")\x00" as *const u8
//                          as *const libc::c_char);
//     } else { };
//     if !(0 as libc::c_int ==
//              buffer_prepend(buf,
//                             b"Hello\x00" as *const u8 as *const libc::c_char
//                                 as *mut libc::c_char)) as libc::c_int as
//            libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 20],
//                                                &[libc::c_char; 20]>(b"test_buffer_prepend\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      80 as libc::c_int,
//                      b"0 == buffer_prepend(buf, \"Hello\")\x00" as *const u8
//                          as *const libc::c_char);
//     } else { };
//     if !(strlen(b"Hello World\x00" as *const u8 as *const libc::c_char) ==
//              buffer_length(buf)) as libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 20],
//                                                &[libc::c_char; 20]>(b"test_buffer_prepend\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      81 as libc::c_int,
//                      b"strlen(\"Hello World\") == buffer_length(buf)\x00" as
//                          *const u8 as *const libc::c_char);
//     } else { };
//     equal(b"Hello World\x00" as *const u8 as *const libc::c_char as
//               *mut libc::c_char, (*buf).data);
//     buffer_free(buf);
// }
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_slice() {
//     let mut buf: *mut buffer_t = buffer_new();
//     buffer_append(buf,
//                   b"Tobi Ferret\x00" as *const u8 as *const libc::c_char);
//     let mut a: *mut buffer_t =
//         buffer_slice(buf, 2 as libc::c_int as size_t,
//                      8 as libc::c_int as ssize_t);
//     equal(b"Tobi Ferret\x00" as *const u8 as *const libc::c_char as
//               *mut libc::c_char, (*buf).data);
//     equal(b"bi Fer\x00" as *const u8 as *const libc::c_char as
//               *mut libc::c_char, (*a).data);
//     buffer_free(buf);
//     buffer_free(a);
// }
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_slice__range_error() {
//     let mut buf: *mut buffer_t =
//         buffer_new_with_copy(b"Tobi Ferret\x00" as *const u8 as
//                                  *const libc::c_char as *mut libc::c_char);
//     let mut a: *mut buffer_t =
//         buffer_slice(buf, 10 as libc::c_int as size_t,
//                      2 as libc::c_int as ssize_t);
//     if !a.is_null() as libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 31],
//                                                &[libc::c_char; 31]>(b"test_buffer_slice__range_error\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      103 as libc::c_int,
//                      b"NULL == a\x00" as *const u8 as *const libc::c_char);
//     } else { };
//     buffer_free(buf);
// }
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_slice__end() {
//     let mut buf: *mut buffer_t =
//         buffer_new_with_copy(b"Tobi Ferret\x00" as *const u8 as
//                                  *const libc::c_char as *mut libc::c_char);
//     let mut a: *mut buffer_t =
//         buffer_slice(buf, 5 as libc::c_int as size_t,
//                      -(1 as libc::c_int) as ssize_t);
//     equal(b"Tobi Ferret\x00" as *const u8 as *const libc::c_char as
//               *mut libc::c_char, (*buf).data);
//     equal(b"Ferret\x00" as *const u8 as *const libc::c_char as
//               *mut libc::c_char, (*a).data);
//     let mut b: *mut buffer_t =
//         buffer_slice(buf, 5 as libc::c_int as size_t,
//                      -(3 as libc::c_int) as ssize_t);
//     equal(b"Ferr\x00" as *const u8 as *const libc::c_char as
//               *mut libc::c_char, (*b).data);
//     let mut c: *mut buffer_t =
//         buffer_slice(buf, 8 as libc::c_int as size_t,
//                      -(1 as libc::c_int) as ssize_t);
//     equal(b"ret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
//           (*c).data);
//     buffer_free(buf);
//     buffer_free(a);
//     buffer_free(b);
//     buffer_free(c);
// }
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_slice__end_overflow() {
//     let mut buf: *mut buffer_t =
//         buffer_new_with_copy(b"Tobi Ferret\x00" as *const u8 as
//                                  *const libc::c_char as *mut libc::c_char);
//     let mut a: *mut buffer_t =
//         buffer_slice(buf, 5 as libc::c_int as size_t,
//                      1000 as libc::c_int as ssize_t);
//     equal(b"Tobi Ferret\x00" as *const u8 as *const libc::c_char as
//               *mut libc::c_char, (*buf).data);
//     equal(b"Ferret\x00" as *const u8 as *const libc::c_char as
//               *mut libc::c_char, (*a).data);
//     buffer_free(a);
//     buffer_free(buf);
// }
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_equals() {
//     let mut a: *mut buffer_t =
//         buffer_new_with_copy(b"Hello\x00" as *const u8 as *const libc::c_char
//                                  as *mut libc::c_char);
//     let mut b: *mut buffer_t =
//         buffer_new_with_copy(b"Hello\x00" as *const u8 as *const libc::c_char
//                                  as *mut libc::c_char);
//     if !(1 as libc::c_int == buffer_equals(a, b)) as libc::c_int as
//            libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 19],
//                                                &[libc::c_char; 19]>(b"test_buffer_equals\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      142 as libc::c_int,
//                      b"1 == buffer_equals(a, b)\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     buffer_append(b, b" World\x00" as *const u8 as *const libc::c_char);
//     if !(0 as libc::c_int == buffer_equals(a, b)) as libc::c_int as
//            libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 19],
//                                                &[libc::c_char; 19]>(b"test_buffer_equals\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      145 as libc::c_int,
//                      b"0 == buffer_equals(a, b)\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     buffer_free(a);
//     buffer_free(b);
// }
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_formatting() {
//     let mut buf: *mut buffer_t = buffer_new();
//     let mut result: libc::c_int =
//         buffer_appendf(buf, b"%d %s\x00" as *const u8 as *const libc::c_char,
//                        3 as libc::c_int,
//                        b"cow\x00" as *const u8 as *const libc::c_char);
//     if !(0 as libc::c_int == result) as libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 23],
//                                                &[libc::c_char; 23]>(b"test_buffer_formatting\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      154 as libc::c_int,
//                      b"0 == result\x00" as *const u8 as *const libc::c_char);
//     } else { };
//     equal(b"3 cow\x00" as *const u8 as *const libc::c_char as
//               *mut libc::c_char, (*buf).data);
//     result =
//         buffer_appendf(buf,
//                        b" - 0x%08X\x00" as *const u8 as *const libc::c_char,
//                        0xdeadbeef as libc::c_uint);
//     if !(0 as libc::c_int == result) as libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 23],
//                                                &[libc::c_char; 23]>(b"test_buffer_formatting\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      157 as libc::c_int,
//                      b"0 == result\x00" as *const u8 as *const libc::c_char);
//     } else { };
//     equal(b"3 cow - 0xDEADBEEF\x00" as *const u8 as *const libc::c_char as
//               *mut libc::c_char, (*buf).data);
//     buffer_free(buf);
// }
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_indexof() {
//     let mut buf: *mut buffer_t =
//         buffer_new_with_copy(b"Tobi is a ferret\x00" as *const u8 as
//                                  *const libc::c_char as *mut libc::c_char);
//     let mut i: ssize_t =
//         buffer_indexof(buf,
//                        b"is\x00" as *const u8 as *const libc::c_char as
//                            *mut libc::c_char);
//     if !(5 as libc::c_int as libc::c_long == i) as libc::c_int as libc::c_long
//            != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 20],
//                                                &[libc::c_char; 20]>(b"test_buffer_indexof\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      167 as libc::c_int,
//                      b"5 == i\x00" as *const u8 as *const libc::c_char);
//     } else { };
//     i =
//         buffer_indexof(buf,
//                        b"a\x00" as *const u8 as *const libc::c_char as
//                            *mut libc::c_char);
//     if !(8 as libc::c_int as libc::c_long == i) as libc::c_int as libc::c_long
//            != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 20],
//                                                &[libc::c_char; 20]>(b"test_buffer_indexof\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      170 as libc::c_int,
//                      b"8 == i\x00" as *const u8 as *const libc::c_char);
//     } else { };
//     i =
//         buffer_indexof(buf,
//                        b"something\x00" as *const u8 as *const libc::c_char as
//                            *mut libc::c_char);
//     if !(-(1 as libc::c_int) as libc::c_long == i) as libc::c_int as
//            libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 20],
//                                                &[libc::c_char; 20]>(b"test_buffer_indexof\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      173 as libc::c_int,
//                      b"-1 == i\x00" as *const u8 as *const libc::c_char);
//     } else { };
//     buffer_free(buf);
// }
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_fill() {
//     let mut buf: *mut buffer_t =
//         buffer_new_with_copy(b"Hello\x00" as *const u8 as *const libc::c_char
//                                  as *mut libc::c_char);
//     if !(5 as libc::c_int as libc::c_ulong == buffer_length(buf)) as
//            libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 17],
//                                                &[libc::c_char; 17]>(b"test_buffer_fill\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      181 as libc::c_int,
//                      b"5 == buffer_length(buf)\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     buffer_fill(buf, 0 as libc::c_int);
//     if !(0 as libc::c_int as libc::c_ulong == buffer_length(buf)) as
//            libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 17],
//                                                &[libc::c_char; 17]>(b"test_buffer_fill\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      184 as libc::c_int,
//                      b"0 == buffer_length(buf)\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     buffer_free(buf);
// }
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_clear() {
//     let mut buf: *mut buffer_t =
//         buffer_new_with_copy(b"Hello\x00" as *const u8 as *const libc::c_char
//                                  as *mut libc::c_char);
//     if !(5 as libc::c_int as libc::c_ulong == buffer_length(buf)) as
//            libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 18],
//                                                &[libc::c_char; 18]>(b"test_buffer_clear\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      191 as libc::c_int,
//                      b"5 == buffer_length(buf)\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     buffer_clear(buf);
//     if !(0 as libc::c_int as libc::c_ulong == buffer_length(buf)) as
//            libc::c_int as libc::c_long != 0 {
//         __assert_rtn((*::std::mem::transmute::<&[u8; 18],
//                                                &[libc::c_char; 18]>(b"test_buffer_clear\x00")).as_ptr(),
//                      b"test.c\x00" as *const u8 as *const libc::c_char,
//                      194 as libc::c_int,
//                      b"0 == buffer_length(buf)\x00" as *const u8 as
//                          *const libc::c_char);
//     } else { };
//     buffer_free(buf);
// }
#[no_mangle]
pub unsafe extern "C" fn test_buffer_trim() {
    let mut buf = buffer_new_with_copy(c_slice!(b"  Hello\n\n"));
    buffer_trim(&mut buf);
    equal(
        b"Hello\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.data_ptr(),
    );
    buffer_free(buf);
    buf = buffer_new_with_copy(c_slice!(b"  Hello\n\n "));
    buffer_trim_left(&mut buf);
    equal(
        b"Hello\n\n \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.data_ptr(),
    );
    buffer_free(buf);
    buf = buffer_new_with_copy(c_slice!(b"  Hello\n\n "),
    );
    buffer_trim_right(&mut buf);
    equal(
        b"  Hello\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.data_ptr(),
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_compact() {
    let mut buf = buffer_new_with_copy(c_slice!(b"  Hello\n\n "));
    buffer_trim(&mut buf);
    assert_eq!(5, buffer_length(&buf));
    assert_eq!(10, buffer_size(&buf));
    let mut removed: ssize_t = buffer_compact(&mut buf);
    assert_eq!(5, removed);
    assert_eq!(5, buffer_length(&buf));
    assert_eq!(5, buffer_size(&buf));
    equal(b"Hello\x00" as *const u8 as *const libc::c_char as
              *mut libc::c_char, buf.data_ptr());
    buffer_free(buf);
}
unsafe fn main_0() -> libc::c_int {
    test_buffer_new();
    test_buffer_new_with_size();
    test_buffer_append();
    // test_buffer_append__grow();
    test_buffer_append_n();
    // test_buffer_prepend();
    // test_buffer_slice();
    // test_buffer_slice__range_error();
    // test_buffer_slice__end();
    // test_buffer_slice__end_overflow();
    // test_buffer_equals();
    // test_buffer_formatting();
    // test_buffer_indexof();
    // test_buffer_fill();
    // test_buffer_clear();
    test_buffer_trim();
    test_buffer_compact();
    printf(c_str!(b"\n  \x1b[32m\xe2\x9c\x93 \x1b[90mok\x1b[0m\n\n").as_ptr());
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
