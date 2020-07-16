use libc;

extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    static mut _DefaultRuneLocale: _RuneLocale;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint32_t = libc::c_uint;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_ssize_t = libc::c_long;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
pub type va_list = __darwin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [libc::c_char; 8],
    pub __encoding: [libc::c_char; 32],
    pub __sgetrune: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: __darwin_size_t,
            _: *mut *const libc::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option<
        unsafe extern "C" fn(
            _: __darwin_rune_t,
            _: *mut libc::c_char,
            _: __darwin_size_t,
            _: *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: libc::c_int,
    pub __ncharclasses: libc::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct buffer_t {
//     pub len: size_t,
//     pub alloc: *mut libc::c_char,
//     pub data: *mut libc::c_char,
// }

pub struct buffer_t {
    pub len: size_t,
    pub alloc: Box<[libc::c_char]>, // freed
    // can't be a `&'a mut[libc::c_char]` -> reference has dynamic lifetime
    // can't be a `RefCell<&'a mut[libc::c_char]>` -> reference *still* has dynamic lifetime
    pub data: size_t, // not freed, can't be CStr cause that contains c_uchar's
}

impl buffer_t {
    pub unsafe fn data_ptr(&self) -> *const libc::c_char {
        self.alloc.as_ptr().offset(self.data as isize)
    }

    pub unsafe fn data_mut_ptr(&mut self) -> *mut libc::c_char {
        self.alloc.as_mut_ptr().offset(self.data as isize)
    }
}

#[inline]
unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
    return (_c & !(0x7f as libc::c_int) == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __istype(mut _c: __darwin_ct_rune_t, mut _f: libc::c_ulong) -> libc::c_int {
    return if isascii(_c) != 0 {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0) as libc::c_int
    } else {
        (__maskrune(_c, _f) != 0) as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isspace(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x4000 as libc::c_long as libc::c_ulong);
}
/*
 * Allocate a new buffer with BUFFER_DEFAULT_SIZE.
 */
pub fn buffer_new<'a>() -> buffer_t {
    return buffer_new_with_size(64 as size_t);
}
/*
 * Allocate a new buffer with `n` bytes.
 */
pub fn buffer_new_with_size<'a>(mut n: size_t) -> buffer_t {
    // If we used MaybeUninit<T>, then we'd have to initialize alloc field in unsafe block
    // before we can call assume_init to get the T out.
    let mut self_0 = buffer_t {
        len: 0,
        alloc: Box::new([0; 0]),
        data: 0,
    };
    self_0.len = n;
    self_0.alloc = vec![0 as libc::c_char; n as usize + 1].into_boxed_slice();
    self_0.data = 0;
    return self_0;
}
/*
 * Allocate a new buffer with `str`.
 */
pub unsafe extern "C" fn buffer_new_with_string(str: Box<[libc::c_char]>) -> buffer_t {
    // note, this has to be a separate stmt, so borrow ends before str is moved into callee
    let len = strlen(str.as_ptr());
    return buffer_new_with_string_length(str, len);
}
/*
 * Allocate a new buffer with `str` and `len`.
 */
pub fn buffer_new_with_string_length(str: Box<[libc::c_char]>, len: size_t) -> buffer_t {
    let mut self_0 = buffer_t {
        len: 0,
        alloc: Box::new([0; 0]),
        data: 0,
    };
    self_0.len = len;
    self_0.alloc = str;
    self_0.data = 0;
    return self_0;
}
/*
 * Allocate a new buffer with a copy of `str`.
 */
#[no_mangle]
pub fn buffer_new_with_copy(mut str: Box<[libc::c_char]>) -> buffer_t {
    let mut len: size_t = unsafe { strlen(str.as_ptr()) };
    let mut self_0: buffer_t = buffer_new_with_size(len);
    self_0.alloc.clone_from_slice(&*str);
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
    buf.clone_from_slice(&*self_0.alloc);
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
    return unsafe { strlen(self_0.data_ptr()) };
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
pub fn buffer_append(mut self_0: &mut buffer_t, str: *const libc::c_char) -> libc::c_int {
    return buffer_append_n(self_0, str, unsafe { strlen(str) });
}
/*
 * Append the first `len` bytes from `str` to `self` and
 * return 0 on success, -1 on failure.
 */
#[no_mangle]
pub fn buffer_append_n(
    self_0: &mut buffer_t,
    str: *const libc::c_char,
    len: size_t,
) -> libc::c_int {
    let mut prev: size_t = unsafe { strlen(self_0.data_ptr()) };
    let mut needed: size_t = len.wrapping_add(prev);
    // enough space
    if self_0.len > needed {
        unsafe { strncat(self_0.data_mut_ptr(), str, len) };
        return 0;
    };
    // resize
    let ret = buffer_resize(self_0, needed);
    if -1 == ret {
        return -1;
    };

    unsafe { strncat(self_0.data_mut_ptr(), str, len) };
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
pub unsafe extern "C" fn buffer_trim_left(mut _self_0: *mut buffer_t) {
    unimplemented!();
    // let mut c: libc::c_int = 0;
    // loop  {
    //     c = *(*self_0).data as libc::c_int;
    //     if !(c != 0 && isspace(c) != 0) { break ; }
    //     (*self_0).data = (*self_0).data.offset(1)
    // };
}
/*
 * Trim trailing whitespace.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_trim_right(mut _self_0: *mut buffer_t) {
    unimplemented!();
    // let mut c: libc::c_int = 0;
    // let mut i: size_t =
    //     buffer_length(self_0).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    // loop  {
    //     c = *(*self_0).data.offset(i as isize) as libc::c_int;
    //     if !(c != 0 && isspace(c) != 0) { break ; }
    //     let fresh0 = i;
    //     i = i.wrapping_sub(1);
    //     *(*self_0).data.offset(fresh0 as isize) =
    //         0 as libc::c_int as libc::c_char
    // };
}
/*
 * Trim trailing and leading whitespace.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_trim(mut self_0: *mut buffer_t) {
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
