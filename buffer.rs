use libc;
use libc::{size_t, ssize_t};
use crate::util;

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
    let len = util::strlen(str.as_ref());
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
    let mut len: size_t = util::strlen(str);
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
#[no_mangle]
pub fn buffer_free(mut _self_0: buffer_t) {
    // free((*self_0).alloc as *mut libc::c_void);
    // free(self_0 as *mut libc::c_void);
}
/*
 * Return buffer size.
 */
#[no_mangle]
pub fn buffer_size(self_0: &buffer_t) -> size_t {
    return self_0.len;
}
/*
 * Return string length.
 */
pub fn buffer_length(self_0: &buffer_t) -> size_t {
    util::strlen(self_0.data_slice())
}

// this was a macro
fn nearest_multiple_of(a: size_t, b: size_t) -> size_t {
    (b + (a - 1)) & !(a - 1)
}
/*
 * Resize to hold `n` bytes.
 */
#[no_mangle]
pub fn buffer_resize(self_0: &mut buffer_t, mut n: size_t) -> libc::c_int {
    n = nearest_multiple_of(1024, n);
    self_0.len = n;
    self_0.data = 0; // TODO: explain how we figure that data needs to be reset
    self_0.alloc = vec![0 as libc::c_char; n as usize + 1].into_boxed_slice();
    // *(*self_0).alloc.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
/*
 * Append a printf-style formatted string to the buffer.
 */
#[no_mangle]
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
#[no_mangle]
pub fn buffer_append(mut self_0: &mut buffer_t, str: &[libc::c_char]) -> libc::c_int {
    return buffer_append_n(self_0, str, util::strlen(str));
}
/*
 * Append the first `len` bytes from `str` to `self` and
 * return 0 on success, -1 on failure.
 */
#[no_mangle]
pub fn buffer_append_n(
    self_0: &mut buffer_t,
    str: &[libc::c_char],
    len: size_t,
) -> libc::c_int {
    let mut prev: size_t = util::strlen(self_0.data_slice());
    let mut needed: size_t = len.wrapping_add(prev);
    // enough space
    if self_0.len > needed {
        util::strncat(self_0.data_mut_slice(), str, len);
        return 0;
    };
    // resize
    let ret = buffer_resize(self_0, needed);
    if -1 == ret {
        return -1;
    };

    util::strncat(self_0.data_mut_slice(), str, len);
    return 0;
}
/*
 * Prepend `str` to `self` and return 0 on success, -1 on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_prepend(
    mut _self_0: *mut buffer_t,
    mut _str: *mut libc::c_char,
) -> libc::c_int {
    unimplemented!();
    // let mut ret: libc::c_int = 0;
    // let mut len: size_t = strlen(str);
    // let mut prev: size_t = strlen((*self_0).data);
    // let mut needed: size_t = len.wrapping_add(prev);
    // // enough space
    // if !((*self_0).len > needed) {
    //     // resize
    //     ret = buffer_resize(self_0, needed);
    //     if -(1 as libc::c_int) == ret { return -(1 as libc::c_int) }
    // }
    // // move
    // memmove((*self_0).data.offset(len as isize) as *mut libc::c_void,
    //         (*self_0).data as *const libc::c_void,
    //         len.wrapping_add(1 as libc::c_int as libc::c_ulong));
    // memcpy((*self_0).data as *mut libc::c_void, str as *const libc::c_void,
    //        len);
    // return 0 as libc::c_int;
}
/*
 * Return a new buffer based on the `from..to` slice of `buf`,
 * or NULL on error.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_slice(
    mut _buf: *mut buffer_t,
    mut _from: size_t,
    mut _to: ssize_t,
) -> *mut buffer_t {
    unimplemented!();
    // let mut len: size_t = strlen((*buf).data);
    // // bad range
    // if (to as libc::c_ulong) < from { return 0 as *mut buffer_t }
    // // relative to end
    // if to < 0 as libc::c_int as libc::c_long {
    //     to = len.wrapping_sub(!to as libc::c_ulong) as ssize_t
    // }
    // // cap end
    // if to as libc::c_ulong > len { to = len as ssize_t }
    // let mut n: size_t = (to as libc::c_ulong).wrapping_sub(from);
    // let mut self_0: *mut buffer_t = buffer_new_with_size(n);
    // memcpy((*self_0).data as *mut libc::c_void,
    //        (*buf).data.offset(from as isize) as *const libc::c_void, n);
    // return self_0;
}
/*
 * Return 1 if the buffers contain equivalent data.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_equals(
    mut _self_0: *mut buffer_t,
    mut _other: *mut buffer_t,
) -> libc::c_int {
    unimplemented!();
    // return (0 as libc::c_int == strcmp((*self_0).data, (*other).data)) as
    //            libc::c_int;
}
/*
 * Return the index of the substring `str`, or -1 on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_indexof(
    mut _self_0: *mut buffer_t,
    mut _str: *mut libc::c_char,
) -> ssize_t {
    unimplemented!();
    // let mut sub: *mut libc::c_char = strstr((*self_0).data, str);
    // if sub.is_null() { return -(1 as libc::c_int) as ssize_t }
    // return sub.wrapping_offset_from((*self_0).data) as libc::c_long;
}
/*
 * Trim leading whitespace.
 */
#[no_mangle]
pub fn buffer_trim_left(self_0: &mut buffer_t) {
    let mut c: libc::c_int = 0;
    loop {
        c = self_0.data_slice()[0] as libc::c_int;
        if !(c != 0 && util::isspace(c) != 0) {
            break;
        }
        self_0.data += 1;
    }
}
/*
 * Trim trailing whitespace.
 */
#[no_mangle]
pub fn buffer_trim_right(mut self_0: &mut buffer_t) {
    let mut c: libc::c_int = 0;
    let mut i: usize = buffer_length(self_0) as usize - 1;
    loop {
        c = self_0.data_slice()[i] as libc::c_int;
        if !(c != 0 && util::isspace(c) != 0) {
            break;
        }
        self_0.data_mut_slice()[i] = 0;
        i = i - 1;
    }
}
/*
 * Trim trailing and leading whitespace.
 */
#[no_mangle]
pub fn buffer_trim(self_0: &mut buffer_t) {
    buffer_trim_left(self_0);
    buffer_trim_right(self_0);
}
/*
 * Fill the buffer with `c`.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_fill(mut _self_0: *mut buffer_t, mut _c: libc::c_int) {
    unimplemented!();
    // memset((*self_0).data as *mut libc::c_void, c, (*self_0).len);
}
/*
 * Fill the buffer with 0.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_clear(mut self_0: *mut buffer_t) {
    buffer_fill(self_0, 0 as libc::c_int);
}
/*
 * Print a hex dump of the buffer.
 */
#[no_mangle]
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
