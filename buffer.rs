use libc;
use std::slice;
use std::ptr::drop_in_place;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: size_t)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> size_t;
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = usize;
pub const _IScntrl: unnamed = 2;
pub const _ISupper: unnamed = 256;
pub const _ISpunct: unnamed = 4;
#[derive ( Clone )]
#[repr(C)]
pub struct buffer_t {
    pub len: size_t,
    pub alloc: Vec<libc::c_char>,
    pub data: *mut libc::c_char,
}
pub const _ISgraph: unnamed = 32768;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const _ISalnum: unnamed = 8;
pub const _ISblank: unnamed = 1;
pub const _ISalpha: unnamed = 1024;
pub const _ISspace: unnamed = 8192;
pub const _ISprint: unnamed = 16384;
pub type ssize_t = __ssize_t;
pub type unnamed = libc::c_uint;
pub const _ISxdigit: unnamed = 4096;
pub const _ISlower: unnamed = 512;
pub const _ISdigit: unnamed = 2048;
#[no_mangle]
pub unsafe extern "C" fn buffer_new() -> *mut buffer_t {
    return buffer_new_with_size(64i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_size(mut n: size_t)
 -> *mut buffer_t {
    let mut self_0: *mut buffer_t =
        malloc(::std::mem::size_of::<buffer_t>()) as
            *mut buffer_t;
    if self_0.is_null() {
        return 0 as *mut buffer_t
    } else {
        (*self_0).len = n;
        (*self_0).alloc = vec![0; n + 1];
        (*self_0).data = (*self_0).alloc.as_mut_ptr();
        return self_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_string(mut str: *mut libc::c_char)
 -> *mut buffer_t {
    return buffer_new_with_string_length(str, strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_string_length(mut str:
                                                           *mut libc::c_char,
                                                       mut len: size_t)
 -> *mut buffer_t {
    let mut self_0: *mut buffer_t =
        malloc(::std::mem::size_of::<buffer_t>()) as
            *mut buffer_t;
    if self_0.is_null() {
        return 0 as *mut buffer_t
    } else {
        (*self_0).len = len;
        (*self_0).alloc = Vec::from_raw_parts(str, len, len);
        (*self_0).data = (*self_0).alloc.as_mut_ptr();
        return self_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_copy(mut str: *mut libc::c_char)
 -> *mut buffer_t {
    let mut len: size_t = strlen(str);
    let mut self_0: *mut buffer_t = buffer_new_with_size(len);
    if self_0.is_null() {
        return 0 as *mut buffer_t
    } else {
        memcpy((*self_0).alloc.as_mut_ptr() as *mut libc::c_void,
               str as *const libc::c_void, len);
        (*self_0).data = (*self_0).alloc.as_mut_ptr();
        return self_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_size(mut self_0: *mut buffer_t) -> size_t {
    return (*self_0).len;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_length(mut self_0: *mut buffer_t) -> size_t {
    return strlen((*self_0).data);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_free(mut self_0: *mut buffer_t) -> () {
    drop_in_place(self_0);
    free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_prepend(mut self_0: *mut buffer_t,
                                        mut str: *mut libc::c_char)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut len: size_t = strlen(str);
    let mut prev: size_t = strlen((*self_0).data);
    let mut needed: size_t = len.wrapping_add(prev);
    if !((*self_0).len > needed) {
        ret = buffer_resize(self_0, needed);
        if -1i32 == ret { return -1i32 }
    }
    memmove((*self_0).data.offset(len as isize) as *mut libc::c_void,
            (*self_0).data as *const libc::c_void,
            len.wrapping_add(1));
    memcpy((*self_0).data as *mut libc::c_void, str as *const libc::c_void,
           len);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_resize(mut self_0: *mut buffer_t,
                                       mut n: size_t) -> libc::c_int {
    n =
        n.wrapping_add(1024 - 1) &
            !(1024 - 1);
    (*self_0).len = n;
    (*self_0).alloc.resize(n.wrapping_add(1), 0);
    (*self_0).data = (*self_0).alloc.as_mut_ptr();
    (*self_0).alloc[n] = '\u{0}' as i32 as libc::c_char;
    return 0i32
}
#[no_mangle]
pub unsafe extern "C" fn buffer_append(mut self_0: *mut buffer_t,
                                       mut str: *const libc::c_char)
 -> libc::c_int {
    return buffer_append_n(self_0, str, strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn buffer_append_n(mut self_0: *mut buffer_t,
                                         mut str: *const libc::c_char,
                                         mut len: size_t) -> libc::c_int {
    let mut prev: size_t = strlen((*self_0).data);
    let mut needed: size_t = len.wrapping_add(prev);
    if (*self_0).len > needed {
        strncat((*self_0).data, str, len);
        return 0i32
    } else {
        let mut ret: libc::c_int = buffer_resize(self_0, needed);
        if -1i32 == ret {
            return -1i32
        } else { strncat((*self_0).data, str, len); return 0i32 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_equals(mut self_0: *mut buffer_t,
                                       mut other: *mut buffer_t)
 -> libc::c_int {
    return (0i32 == strcmp((*self_0).data, (*other).data)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_indexof(mut self_0: *mut buffer_t,
                                        mut str: *mut libc::c_char)
 -> ssize_t {
    let mut sub: *mut libc::c_char = strstr((*self_0).data, str);
    if sub.is_null() {
        return -1i32 as ssize_t
    } else {
        return (*self_0).data.offset_to(sub).expect("bad offset_to") as
                   libc::c_long
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_slice(mut buf: *mut buffer_t,
                                      mut from: size_t, mut to: ssize_t)
 -> *mut buffer_t {
    let mut len: size_t = strlen((*buf).data);
    if (to as size_t) < from {
        return 0 as *mut buffer_t
    } else {
        if to < 0i32 as libc::c_long {
            to = len.wrapping_sub(!to as size_t) as ssize_t
        }
        if to as size_t > len { to = len as ssize_t }
        let mut n: size_t = (to as size_t).wrapping_sub(from);
        let mut self_0: *mut buffer_t = buffer_new_with_size(n);
        memcpy((*self_0).data as *mut libc::c_void,
               (*buf).data.offset(from as isize) as *const libc::c_void, n);
        return self_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_compact(mut self_0: *mut buffer_t)
 -> ssize_t {
    let mut len: size_t = buffer_length(self_0);
    let mut rem: size_t = (*self_0).len.wrapping_sub(len);
    let mut buf = vec![0; len + 1];
    buf.splice(..len, slice::from_raw_parts((*self_0).data, len).iter().cloned());
    (*self_0).len = len;
    (*self_0).alloc = buf;
    (*self_0).data = (*self_0).alloc.as_mut_ptr();
    return rem as ssize_t
}
#[no_mangle]
pub unsafe extern "C" fn buffer_fill(mut self_0: *mut buffer_t,
                                     mut c: libc::c_int) -> () {
    memset((*self_0).data as *mut libc::c_void, c, (*self_0).len);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_clear(mut self_0: *mut buffer_t) -> () {
    buffer_fill(self_0, 0i32);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_trim_left(mut self_0: *mut buffer_t) -> () {
    let mut c: libc::c_int = 0;
    loop  {
        c = *(*self_0).data as libc::c_int;
        if !(0 != c &&
                 0 !=
                     *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                         _ISspace as libc::c_int as libc::c_ushort as
                             libc::c_int) {
            break ;
        }
        (*self_0).data = (*self_0).data.offset(1isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_trim_right(mut self_0: *mut buffer_t) -> () {
    let mut c: libc::c_int = 0;
    let mut i: size_t =
        buffer_length(self_0).wrapping_sub(1);
    loop  {
        c = *(*self_0).data.offset(i as isize) as libc::c_int;
        if !(0 != c &&
                 0 !=
                     *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                         _ISspace as libc::c_int as libc::c_ushort as
                             libc::c_int) {
            break ;
        }
        let fresh0 = i;
        i = i.wrapping_sub(1);
        *(*self_0).data.offset(fresh0 as isize) = 0i32 as libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_trim(mut self_0: *mut buffer_t) -> () {
    buffer_trim_left(self_0);
    buffer_trim_right(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_print(mut self_0: *mut buffer_t) -> () {
    let mut i: libc::c_int = 0;
    let mut len: size_t = (*self_0).len;
    printf((*::std::mem::transmute::<&[u8; 3],
                                     &mut [libc::c_char; 3]>(b"\n \x00")).as_mut_ptr());
    i = 0i32;
    while (i as size_t) < len {
        printf((*::std::mem::transmute::<&[u8; 6],
                                         &mut [libc::c_char; 6]>(b" %02x\x00")).as_mut_ptr(),
               &(*self_0).alloc[i as usize]);
        if (i + 1i32) % 8i32 == 0i32 {
            printf((*::std::mem::transmute::<&[u8; 3],
                                             &mut [libc::c_char; 3]>(b"\n \x00")).as_mut_ptr());
        }
        i += 1
    }
    printf((*::std::mem::transmute::<&[u8; 2],
                                     &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
}
