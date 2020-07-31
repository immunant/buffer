use libc;
use libc::{size_t, ssize_t};
use crate::util::*;
use std::os::raw::c_int;
use std::convert::TryInto;

const BUFFER_DEFAULT_SIZE: size_t = 64;

pub struct buffer_t {
    pub len: size_t,
    pub alloc: Box<[libc::c_char]>,
    pub data: size_t, // points to first char of string in `alloc`
}

impl buffer_t {
    pub unsafe fn data_ptr(&self) -> *const libc::c_char {
        self.alloc.as_ptr().offset(self.data as isize)
    }

    pub unsafe fn data_mut_ptr(&mut self) -> *mut libc::c_char {
        self.alloc.as_mut_ptr().offset(self.data as isize)
    }

    pub fn data_slice(&self) -> &[libc::c_char] {
        &self.alloc[self.data as usize..]
    }

    pub fn data_mut_slice(&mut self) -> &mut [libc::c_char] {
        &mut self.alloc[self.data as usize..]
    }
}

/*
 * Allocate a new buffer with BUFFER_DEFAULT_SIZE.
 */
pub fn buffer_new() -> buffer_t {
    return buffer_new_with_size(BUFFER_DEFAULT_SIZE);
}
/*
 * Allocate a new buffer with `n` bytes.
 */
pub fn buffer_new_with_size(mut n: size_t) -> buffer_t {
    let mut self_0 = buffer_t { len: 0, alloc: Box::new([0; 0]), data: 0 };
    self_0.len = n;
    self_0.alloc = vec![0 as libc::c_char; n as usize + 1].into_boxed_slice();
    self_0.data = 0;
    self_0
}
/*
 * Allocate a new buffer with `str`.
 */
pub fn buffer_new_with_string(str: Box<[libc::c_char]>) -> buffer_t {
    let len = strlen(str.as_ref());
    return buffer_new_with_string_length(str, len as size_t);
}
/*
 * Allocate a new buffer with `str` and `len`.
 */
pub fn buffer_new_with_string_length(str: Box<[libc::c_char]>, len: size_t) -> buffer_t {
    let mut self_0 = buffer_t { len: 0, alloc: Box::new([0; 0]), data: 0 };
    self_0.len = len;
    self_0.alloc = str;
    self_0.data = 0;
    self_0
}
/*
 * Allocate a new buffer with a copy of `str`.
 */
#[no_mangle]
pub fn buffer_new_with_copy(mut str: &[libc::c_char]) -> buffer_t {
    let mut len: size_t = strlen(str);
    let mut self_0: buffer_t = buffer_new_with_size(len);
    self_0.alloc.clone_from_slice(str);
    self_0.data = 0;
    return self_0;
}
/*
 * Deallocate excess memory, the number
 * of bytes removed or -1.
 */
pub fn buffer_compact(self_0: &mut buffer_t) -> ssize_t {
    let len: size_t = buffer_length(self_0);
    let rem: size_t = self_0.len.wrapping_sub(len);
    let mut buf = vec![0; len as usize + 1].into_boxed_slice();
    let t = self_0.data_slice();
    buf.clone_from_slice(&t[..len as usize + 1]);
    self_0.len = len;
    self_0.alloc = buf;
    self_0.data = 0;
    return rem as ssize_t;
}
/*
 * Free the buffer.
 */
pub fn buffer_free(mut _self_0: buffer_t) {
    // free((*self_0).alloc as *mut libc::c_void);
    // free(self_0 as *mut libc::c_void);
}
/*
 * Return buffer size.
 */
pub fn buffer_size(self_0: &buffer_t) -> size_t {
    return self_0.len;
}
/*
 * Return string length.
 */
pub fn buffer_length(self_0: &buffer_t) -> size_t {
    strlen(self_0.data_slice())
}

// this was a macro
fn nearest_multiple_of(a: size_t, b: size_t) -> size_t {
    (b + (a - 1)) & !(a - 1)
}
/*
 * Resize to hold `n` bytes.
 */
pub fn buffer_resize(
    self_0: &mut buffer_t,
    mut n: size_t) -> libc::c_int {
    n = nearest_multiple_of(1024, n);
    self_0.len = n;
    self_0.data = 0;
    let mut t = self_0.alloc.to_vec();
    t.resize_with(n + 1, Default::default);
    self_0.alloc = t.into_boxed_slice();
    // NOTE: this statement is superfluous:
    self_0.alloc[n] = 0;
    return 0;
}
/*
 * Append a printf-style formatted string to the buffer.
 */
pub unsafe extern "C" fn buffer_appendf(
    mut _self_0: *mut buffer_t,
    mut _format: *const libc::c_char,
    mut _args: ...
) -> libc::c_int {
    unimplemented!();
    //     let mut ap: ::std::ffi::VaListImpl;
    //     let mut tmpa: ::std::ffi::VaListImpl;
    //     let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    //     let mut length: libc::c_int = 0 as libc::c_int;
    //     let mut required: libc::c_int = 0 as libc::c_int;
    //     let mut bytes: libc::c_int = 0 as libc::c_int;
    //     ap = args.clone();
    //     length = buffer_length(self_0) as libc::c_int;
    //     // First, we compute how many bytes are needed
    //   // for the formatted string and allocate that
    //   // much more space in the buffer.
    //     tmpa = ap.clone();
    //     required =
    //         vsnprintf(0 as *mut libc::c_char, 0 as libc::c_int as libc::c_ulong,
    //                   format, tmpa.as_va_list());
    //     if -(1 as libc::c_int) ==
    //            buffer_resize(self_0, (length + required) as size_t) {
    //         return -(1 as libc::c_int)
    //     }
    //     // Next format the string into the space that we
    //   // have made room for.
    //     dst = (*self_0).data.offset(length as isize);
    //     bytes =
    //         vsnprintf(dst, (1 as libc::c_int + required) as libc::c_ulong, format,
    //                   ap.as_va_list());
    //     return if bytes < 0 as libc::c_int {
    //                -(1 as libc::c_int)
    //            } else { 0 as libc::c_int };
}
/*
 * Append `str` to `self` and return 0 on success, -1 on failure.
 */
pub fn buffer_append(mut self_0: &mut buffer_t, str: &[libc::c_char]) -> libc::c_int {
    return buffer_append_n(self_0, str, strlen(str));
}
/*
 * Append the first `len` bytes from `str` to `self` and
 * return 0 on success, -1 on failure.
 */
pub fn buffer_append_n(
    self_0: &mut buffer_t,
    str: &[libc::c_char],
    len: size_t,
) -> libc::c_int {
    let mut prev: size_t = strlen(self_0.data_slice());
    let mut needed: size_t = len.wrapping_add(prev);
    // enough space
    if self_0.len > needed {
        strncat(self_0.data_mut_slice(), str, len);
        return 0;
    };
    // resize
    let ret = buffer_resize(self_0, needed);
    if -1 == ret {
        return -1;
    };

    strncat(self_0.data_mut_slice(), str, len);
    return 0;
}
/*
 * Prepend `str` to `self` and return 0 on success, -1 on failure.
 */
pub fn buffer_prepend(
    mut self_0: &mut buffer_t,
    mut str: &[libc::c_char],
) -> libc::c_int {
    let mut len: size_t = strlen(str);
    let mut prev: size_t = strlen(self_0.data_slice());
    let mut needed: size_t = len.wrapping_add(prev);
    // enough space
    if !(self_0.len > needed) {
        // resize
        let ret = buffer_resize(&mut self_0, needed);
        if -1 == ret { return -1 }
    }
    // move
    unsafe {
        libc::memmove(self_0.data_ptr().offset(len as isize) as *mut libc::c_void,
                self_0.data_ptr() as *const libc::c_void,
                len + 1);
        libc::memcpy(self_0.data_ptr() as *mut libc::c_void,
                     str.as_ptr() as *const libc::c_void,
               len);
    }
    return 0 as libc::c_int;
}
/*
 * Return a new buffer based on the `from..to` slice of `buf`,
 * or NULL on error.
 */
pub fn buffer_slice(
    mut buf: &buffer_t,
    mut from: size_t,
    mut to: ssize_t,
) -> Option<buffer_t> {
    let mut len: size_t = strlen(buf.data_slice());
    // bad range
    if (to as size_t) < from { return None }
    // relative to end
    if to < 0 {
        to = (len - (!to as usize)) as ssize_t
    }
    // cap end
    if to as size_t > len { to = len as ssize_t }
    let mut n: size_t = (to as size_t)- from;
    let mut self_0 = buffer_new_with_size(n);
    let src = &buf.data_slice()[from..from + n];
    let dst = &mut self_0.data_mut_slice()[..n];
    dst.copy_from_slice(src);
    return Some(self_0);
}
/*
 * Return 1 if the buffers contain equivalent data.
 */
pub fn buffer_equals(mut self_0: &buffer_t, mut other: &buffer_t) -> libc::c_int {
    (strcmp(self_0.data_slice(), other.data_slice()) == 0) as c_int
}
/*
 * Return the index of the substring `str`, or -1 on failure.
 */
pub fn buffer_indexof(
    self_0: &buffer_t,
    str: &[libc::c_char],
) -> ssize_t {
    let mut sub = strstr(self_0.data_slice(), str);
    if sub.is_none() { return -(1 as libc::c_int) as ssize_t }
    return sub.unwrap().wrapping_sub(self_0.data) as ssize_t;
}
/*
 * Trim leading whitespace.
 */
pub fn buffer_trim_left(self_0: &mut buffer_t) {
    loop {
        let mut c = self_0.data_slice()[0] as libc::c_int;
        if !(c != 0 && isspace(c) != 0) {
            break;
        }
        self_0.data += 1;
    }
}
/*
 * Trim trailing whitespace.
 */
pub fn buffer_trim_right(mut self_0: &mut buffer_t) {
    let mut c: libc::c_int = 0;
    let mut i: usize = buffer_length(self_0) as usize - 1;
    loop {
        c = self_0.data_slice()[i] as libc::c_int;
        if !(c != 0 && isspace(c) != 0) {
            break;
        }
        self_0.data_mut_slice()[i] = 0;
        i = i - 1;
    }
}
/*
 * Trim trailing and leading whitespace.
 */
pub fn buffer_trim(self_0: &mut buffer_t) {
    buffer_trim_left(self_0);
    buffer_trim_right(self_0);
}
/*
 * Fill the buffer with `c`.
 */
pub fn buffer_fill(mut self_0: &mut buffer_t, mut c: libc::c_int) {
    // NOTE: memset fills with a i32, fill takes an i8 so we use try_into.
    // memset((*self_0).data as *mut libc::c_void, c, (*self_0).len);
    self_0.data_mut_slice().fill(c.try_into().unwrap());

}
/*
 * Fill the buffer with 0.
 */
#[no_mangle]
pub fn buffer_clear(mut self_0: &mut buffer_t) {
    buffer_fill(self_0, 0);
}
/*
 * Print a hex dump of the buffer.
 */
pub unsafe extern "C" fn buffer_print(mut _self_0: *mut buffer_t) {
    unimplemented!();
    // let mut i: libc::c_int = 0;
    // let mut len: size_t = (*self_0).len;
    // printf(b"\n \x00" as *const u8 as *const libc::c_char);
    // // hex
    // i = 0 as libc::c_int;
    // while (i as libc::c_ulong) < len {
    //     printf(b" %02x\x00" as *const u8 as *const libc::c_char,
    //            *(*self_0).alloc.offset(i as isize) as libc::c_int);
    //     if (i + 1 as libc::c_int) % 8 as libc::c_int == 0 as libc::c_int {
    //         printf(b"\n \x00" as *const u8 as *const libc::c_char);
    //     }
    //     i += 1
    // }
    // printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
