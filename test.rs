use libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn buffer_new() -> *mut buffer_t;
    #[no_mangle]
    fn buffer_new_with_size(n: size_t) -> *mut buffer_t;
    #[no_mangle]
    fn buffer_new_with_copy(str: *mut libc::c_char) -> *mut buffer_t;
    #[no_mangle]
    fn buffer_size(self_0: *mut buffer_t) -> size_t;
    #[no_mangle]
    fn buffer_length(self_0: *mut buffer_t) -> size_t;
    #[no_mangle]
    fn buffer_free(self_0: *mut buffer_t) -> ();
    #[no_mangle]
    fn buffer_prepend(self_0: *mut buffer_t, str: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn buffer_append(self_0: *mut buffer_t, str: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn buffer_append_n(self_0: *mut buffer_t, str: *const libc::c_char,
                       len: size_t) -> libc::c_int;
    #[no_mangle]
    fn buffer_equals(self_0: *mut buffer_t, other: *mut buffer_t)
     -> libc::c_int;
    #[no_mangle]
    fn buffer_indexof(self_0: *mut buffer_t, str: *mut libc::c_char)
     -> ssize_t;
    #[no_mangle]
    fn buffer_slice(self_0: *mut buffer_t, from: size_t, to: ssize_t)
     -> *mut buffer_t;
    #[no_mangle]
    fn buffer_compact(self_0: *mut buffer_t) -> ssize_t;
    #[no_mangle]
    fn buffer_fill(self_0: *mut buffer_t, c: libc::c_int) -> ();
    #[no_mangle]
    fn buffer_clear(self_0: *mut buffer_t) -> ();
    #[no_mangle]
    fn buffer_trim_left(self_0: *mut buffer_t) -> ();
    #[no_mangle]
    fn buffer_trim_right(self_0: *mut buffer_t) -> ();
    #[no_mangle]
    fn buffer_trim(self_0: *mut buffer_t) -> ();
}
pub type size_t = libc::c_ulong;
#[derive ( Clone )]
#[repr(C)]
pub struct buffer_t {
    pub len: size_t,
    pub alloc: Vec<libc::c_char>,
    pub data: *mut libc::c_char,
}
pub type ssize_t = __ssize_t;
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
#[no_mangle]
pub unsafe extern "C" fn equal(mut a: *mut libc::c_char,
                               mut b: *mut libc::c_char) -> () {
    if 0 != strcmp(a, b) {
        printf((*::std::mem::transmute::<&[u8; 2],
                                         &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
        printf((*::std::mem::transmute::<&[u8; 18],
                                         &mut [libc::c_char; 18]>(b"  expected: \'%s\'\n\x00")).as_mut_ptr(),
               a);
        printf((*::std::mem::transmute::<&[u8; 18],
                                         &mut [libc::c_char; 18]>(b"    actual: \'%s\'\n\x00")).as_mut_ptr(),
               b);
        printf((*::std::mem::transmute::<&[u8; 2],
                                         &mut [libc::c_char; 2]>(b"\n\x00")).as_mut_ptr());
        exit(1i32);
    } else { return; };
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_new() -> () {
    let mut buf: *mut buffer_t = buffer_new();
    if 64i32 as libc::c_ulong == buffer_size(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 40],
                                                &mut [libc::c_char; 40]>(b"BUFFER_DEFAULT_SIZE == buffer_size(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      28i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"void test_buffer_new()\x00")).as_ptr());
    };
    if 0i32 as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 24],
                                                &mut [libc::c_char; 24]>(b"0 == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      29i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"void test_buffer_new()\x00")).as_ptr());
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_new_with_size() -> () {
    let mut buf: *mut buffer_t = buffer_new_with_size(1024i32 as size_t);
    if 1024i32 as libc::c_ulong == buffer_size(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 25],
                                                &mut [libc::c_char; 25]>(b"1024 == buffer_size(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      36i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void test_buffer_new_with_size()\x00")).as_ptr());
    };
    if 0i32 as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 24],
                                                &mut [libc::c_char; 24]>(b"0 == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      37i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void test_buffer_new_with_size()\x00")).as_ptr());
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append() -> () {
    let mut buf: *mut buffer_t = buffer_new();
    if 0i32 ==
           buffer_append(buf,
                         (*::std::mem::transmute::<&[u8; 6],
                                                   &mut [libc::c_char; 6]>(b"Hello\x00")).as_mut_ptr())
       {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 33],
                                                &mut [libc::c_char; 33]>(b"0 == buffer_append(buf, \"Hello\")\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      44i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void test_buffer_append()\x00")).as_ptr());
    };
    if 0i32 ==
           buffer_append(buf,
                         (*::std::mem::transmute::<&[u8; 7],
                                                   &mut [libc::c_char; 7]>(b" World\x00")).as_mut_ptr())
       {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 34],
                                                &mut [libc::c_char; 34]>(b"0 == buffer_append(buf, \" World\")\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      45i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void test_buffer_append()\x00")).as_ptr());
    };
    if strlen((*::std::mem::transmute::<&[u8; 12],
                                        &mut [libc::c_char; 12]>(b"Hello World\x00")).as_mut_ptr())
           == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 44],
                                                &mut [libc::c_char; 44]>(b"strlen(\"Hello World\") == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      46i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void test_buffer_append()\x00")).as_ptr());
    };
    equal((*::std::mem::transmute::<&[u8; 12],
                                    &mut [libc::c_char; 12]>(b"Hello World\x00")).as_mut_ptr(),
          (*buf).data);
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append_n() -> () {
    let mut buf: *mut buffer_t = buffer_new();
    if 0i32 ==
           buffer_append_n(buf,
                           (*::std::mem::transmute::<&[u8; 7],
                                                     &mut [libc::c_char; 7]>(b"subway\x00")).as_mut_ptr(),
                           3i32 as size_t) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 39],
                                                &mut [libc::c_char; 39]>(b"0 == buffer_append_n(buf, \"subway\", 3)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      54i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"void test_buffer_append_n()\x00")).as_ptr());
    };
    if 0i32 ==
           buffer_append_n(buf,
                           (*::std::mem::transmute::<&[u8; 8],
                                                     &mut [libc::c_char; 8]>(b"marines\x00")).as_mut_ptr(),
                           6i32 as size_t) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 40],
                                                &mut [libc::c_char; 40]>(b"0 == buffer_append_n(buf, \"marines\", 6)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      55i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"void test_buffer_append_n()\x00")).as_ptr());
    };
    if strlen((*::std::mem::transmute::<&[u8; 10],
                                        &mut [libc::c_char; 10]>(b"submarine\x00")).as_mut_ptr())
           == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 42],
                                                &mut [libc::c_char; 42]>(b"strlen(\"submarine\") == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      56i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 28],
                                                &[libc::c_char; 28]>(b"void test_buffer_append_n()\x00")).as_ptr());
    };
    equal((*::std::mem::transmute::<&[u8; 10],
                                    &mut [libc::c_char; 10]>(b"submarine\x00")).as_mut_ptr(),
          (*buf).data);
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append__grow() -> () {
    let mut buf: *mut buffer_t = buffer_new_with_size(10i32 as size_t);
    if 0i32 ==
           buffer_append(buf,
                         (*::std::mem::transmute::<&[u8; 6],
                                                   &mut [libc::c_char; 6]>(b"Hello\x00")).as_mut_ptr())
       {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 33],
                                                &mut [libc::c_char; 33]>(b"0 == buffer_append(buf, \"Hello\")\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      64i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void test_buffer_append__grow()\x00")).as_ptr());
    };
    if 0i32 ==
           buffer_append(buf,
                         (*::std::mem::transmute::<&[u8; 6],
                                                   &mut [libc::c_char; 6]>(b" tobi\x00")).as_mut_ptr())
       {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 33],
                                                &mut [libc::c_char; 33]>(b"0 == buffer_append(buf, \" tobi\")\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      65i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void test_buffer_append__grow()\x00")).as_ptr());
    };
    if 0i32 ==
           buffer_append(buf,
                         (*::std::mem::transmute::<&[u8; 5],
                                                   &mut [libc::c_char; 5]>(b" was\x00")).as_mut_ptr())
       {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 32],
                                                &mut [libc::c_char; 32]>(b"0 == buffer_append(buf, \" was\")\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      66i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void test_buffer_append__grow()\x00")).as_ptr());
    };
    if 0i32 ==
           buffer_append(buf,
                         (*::std::mem::transmute::<&[u8; 6],
                                                   &mut [libc::c_char; 6]>(b" here\x00")).as_mut_ptr())
       {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 33],
                                                &mut [libc::c_char; 33]>(b"0 == buffer_append(buf, \" here\")\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      67i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void test_buffer_append__grow()\x00")).as_ptr());
    };
    let mut str: *mut libc::c_char =
        (*::std::mem::transmute::<&[u8; 20],
                                  &mut [libc::c_char; 20]>(b"Hello tobi was here\x00")).as_mut_ptr();
    equal(str, (*buf).data);
    if 1024i32 as libc::c_ulong == buffer_size(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 25],
                                                &mut [libc::c_char; 25]>(b"1024 == buffer_size(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      71i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void test_buffer_append__grow()\x00")).as_ptr());
    };
    if strlen(str) == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 34],
                                                &mut [libc::c_char; 34]>(b"strlen(str) == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      72i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void test_buffer_append__grow()\x00")).as_ptr());
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_prepend() -> () {
    let mut buf: *mut buffer_t = buffer_new();
    if 0i32 ==
           buffer_append(buf,
                         (*::std::mem::transmute::<&[u8; 7],
                                                   &mut [libc::c_char; 7]>(b" World\x00")).as_mut_ptr())
       {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 34],
                                                &mut [libc::c_char; 34]>(b"0 == buffer_append(buf, \" World\")\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      79i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_prepend()\x00")).as_ptr());
    };
    if 0i32 ==
           buffer_prepend(buf,
                          (*::std::mem::transmute::<&[u8; 6],
                                                    &mut [libc::c_char; 6]>(b"Hello\x00")).as_mut_ptr())
       {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 34],
                                                &mut [libc::c_char; 34]>(b"0 == buffer_prepend(buf, \"Hello\")\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      80i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_prepend()\x00")).as_ptr());
    };
    if strlen((*::std::mem::transmute::<&[u8; 12],
                                        &mut [libc::c_char; 12]>(b"Hello World\x00")).as_mut_ptr())
           == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 44],
                                                &mut [libc::c_char; 44]>(b"strlen(\"Hello World\") == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      81i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_prepend()\x00")).as_ptr());
    };
    equal((*::std::mem::transmute::<&[u8; 12],
                                    &mut [libc::c_char; 12]>(b"Hello World\x00")).as_mut_ptr(),
          (*buf).data);
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice() -> () {
    let mut buf: *mut buffer_t = buffer_new();
    buffer_append(buf,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &mut [libc::c_char; 12]>(b"Tobi Ferret\x00")).as_mut_ptr());
    let mut a: *mut buffer_t =
        buffer_slice(buf, 2i32 as size_t, 8i32 as ssize_t);
    equal((*::std::mem::transmute::<&[u8; 12],
                                    &mut [libc::c_char; 12]>(b"Tobi Ferret\x00")).as_mut_ptr(),
          (*buf).data);
    equal((*::std::mem::transmute::<&[u8; 7],
                                    &mut [libc::c_char; 7]>(b"bi Fer\x00")).as_mut_ptr(),
          (*a).data);
    buffer_free(buf);
    buffer_free(a);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__range_error() -> () {
    let mut buf: *mut buffer_t =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 12],
                                                       &mut [libc::c_char; 12]>(b"Tobi Ferret\x00")).as_mut_ptr());
    let mut a: *mut buffer_t =
        buffer_slice(buf, 10i32 as size_t, 2i32 as ssize_t);
    if a.is_null() {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 10],
                                                &mut [libc::c_char; 10]>(b"NULL == a\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      103i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 38],
                                                &[libc::c_char; 38]>(b"void test_buffer_slice__range_error()\x00")).as_ptr());
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__end() -> () {
    let mut buf: *mut buffer_t =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 12],
                                                       &mut [libc::c_char; 12]>(b"Tobi Ferret\x00")).as_mut_ptr());
    let mut a: *mut buffer_t =
        buffer_slice(buf, 5i32 as size_t, -1i32 as ssize_t);
    equal((*::std::mem::transmute::<&[u8; 12],
                                    &mut [libc::c_char; 12]>(b"Tobi Ferret\x00")).as_mut_ptr(),
          (*buf).data);
    equal((*::std::mem::transmute::<&[u8; 7],
                                    &mut [libc::c_char; 7]>(b"Ferret\x00")).as_mut_ptr(),
          (*a).data);
    let mut b: *mut buffer_t =
        buffer_slice(buf, 5i32 as size_t, -3i32 as ssize_t);
    equal((*::std::mem::transmute::<&[u8; 5],
                                    &mut [libc::c_char; 5]>(b"Ferr\x00")).as_mut_ptr(),
          (*b).data);
    let mut c: *mut buffer_t =
        buffer_slice(buf, 8i32 as size_t, -1i32 as ssize_t);
    equal((*::std::mem::transmute::<&[u8; 4],
                                    &mut [libc::c_char; 4]>(b"ret\x00")).as_mut_ptr(),
          (*c).data);
    buffer_free(buf);
    buffer_free(a);
    buffer_free(b);
    buffer_free(c);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__end_overflow() -> () {
    let mut buf: *mut buffer_t =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 12],
                                                       &mut [libc::c_char; 12]>(b"Tobi Ferret\x00")).as_mut_ptr());
    let mut a: *mut buffer_t =
        buffer_slice(buf, 5i32 as size_t, 1000i32 as ssize_t);
    equal((*::std::mem::transmute::<&[u8; 12],
                                    &mut [libc::c_char; 12]>(b"Tobi Ferret\x00")).as_mut_ptr(),
          (*buf).data);
    equal((*::std::mem::transmute::<&[u8; 7],
                                    &mut [libc::c_char; 7]>(b"Ferret\x00")).as_mut_ptr(),
          (*a).data);
    buffer_free(a);
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_equals() -> () {
    let mut a: *mut buffer_t =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 6],
                                                       &mut [libc::c_char; 6]>(b"Hello\x00")).as_mut_ptr());
    let mut b: *mut buffer_t =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 6],
                                                       &mut [libc::c_char; 6]>(b"Hello\x00")).as_mut_ptr());
    if 1i32 == buffer_equals(a, b) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 25],
                                                &mut [libc::c_char; 25]>(b"1 == buffer_equals(a, b)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      142i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void test_buffer_equals()\x00")).as_ptr());
    };
    buffer_append(b,
                  (*::std::mem::transmute::<&[u8; 7],
                                            &mut [libc::c_char; 7]>(b" World\x00")).as_mut_ptr());
    if 0i32 == buffer_equals(a, b) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 25],
                                                &mut [libc::c_char; 25]>(b"0 == buffer_equals(a, b)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      145i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"void test_buffer_equals()\x00")).as_ptr());
    };
    buffer_free(a);
    buffer_free(b);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_indexof() -> () {
    let mut buf: *mut buffer_t =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 17],
                                                       &mut [libc::c_char; 17]>(b"Tobi is a ferret\x00")).as_mut_ptr());
    let mut i: ssize_t =
        buffer_indexof(buf,
                       (*::std::mem::transmute::<&[u8; 3],
                                                 &mut [libc::c_char; 3]>(b"is\x00")).as_mut_ptr());
    if 5i32 as libc::c_long == i {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"5 == i\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      169i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_indexof()\x00")).as_ptr());
    };
    i =
        buffer_indexof(buf,
                       (*::std::mem::transmute::<&[u8; 2],
                                                 &mut [libc::c_char; 2]>(b"a\x00")).as_mut_ptr());
    if 8i32 as libc::c_long == i {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"8 == i\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      172i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_indexof()\x00")).as_ptr());
    };
    i =
        buffer_indexof(buf,
                       (*::std::mem::transmute::<&[u8; 10],
                                                 &mut [libc::c_char; 10]>(b"something\x00")).as_mut_ptr());
    if -1i32 as libc::c_long == i {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 8],
                                                &mut [libc::c_char; 8]>(b"-1 == i\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      175i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_indexof()\x00")).as_ptr());
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_fill() -> () {
    let mut buf: *mut buffer_t =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 6],
                                                       &mut [libc::c_char; 6]>(b"Hello\x00")).as_mut_ptr());
    if 5i32 as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 24],
                                                &mut [libc::c_char; 24]>(b"5 == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      183i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"void test_buffer_fill()\x00")).as_ptr());
    };
    buffer_fill(buf, 0i32);
    if 0i32 as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 24],
                                                &mut [libc::c_char; 24]>(b"0 == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      186i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"void test_buffer_fill()\x00")).as_ptr());
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_clear() -> () {
    let mut buf: *mut buffer_t =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 6],
                                                       &mut [libc::c_char; 6]>(b"Hello\x00")).as_mut_ptr());
    if 5i32 as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 24],
                                                &mut [libc::c_char; 24]>(b"5 == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      193i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"void test_buffer_clear()\x00")).as_ptr());
    };
    buffer_clear(buf);
    if 0i32 as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 24],
                                                &mut [libc::c_char; 24]>(b"0 == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      196i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"void test_buffer_clear()\x00")).as_ptr());
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_trim() -> () {
    let mut buf: *mut buffer_t =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 11],
                                                       &mut [libc::c_char; 11]>(b"  Hello\n\n \x00")).as_mut_ptr());
    buffer_trim(buf);
    equal((*::std::mem::transmute::<&[u8; 6],
                                    &mut [libc::c_char; 6]>(b"Hello\x00")).as_mut_ptr(),
          (*buf).data);
    buffer_free(buf);
    buf =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 11],
                                                       &mut [libc::c_char; 11]>(b"  Hello\n\n \x00")).as_mut_ptr());
    buffer_trim_left(buf);
    equal((*::std::mem::transmute::<&[u8; 9],
                                    &mut [libc::c_char; 9]>(b"Hello\n\n \x00")).as_mut_ptr(),
          (*buf).data);
    buffer_free(buf);
    buf =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 11],
                                                       &mut [libc::c_char; 11]>(b"  Hello\n\n \x00")).as_mut_ptr());
    buffer_trim_right(buf);
    equal((*::std::mem::transmute::<&[u8; 8],
                                    &mut [libc::c_char; 8]>(b"  Hello\x00")).as_mut_ptr(),
          (*buf).data);
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_compact() -> () {
    let mut buf: *mut buffer_t =
        buffer_new_with_copy((*::std::mem::transmute::<&[u8; 11],
                                                       &mut [libc::c_char; 11]>(b"  Hello\n\n \x00")).as_mut_ptr());
    buffer_trim(buf);
    if 5i32 as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 24],
                                                &mut [libc::c_char; 24]>(b"5 == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      222i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_compact()\x00")).as_ptr());
    };
    if 10i32 as libc::c_ulong == buffer_size(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 23],
                                                &mut [libc::c_char; 23]>(b"10 == buffer_size(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      223i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_compact()\x00")).as_ptr());
    };
    let mut removed: ssize_t = buffer_compact(buf);
    if 5i32 as libc::c_long == removed {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 13],
                                                &mut [libc::c_char; 13]>(b"5 == removed\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      226i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_compact()\x00")).as_ptr());
    };
    if 5i32 as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 24],
                                                &mut [libc::c_char; 24]>(b"5 == buffer_length(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      227i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_compact()\x00")).as_ptr());
    };
    if 5i32 as libc::c_ulong == buffer_size(buf) {
    } else {
        __assert_fail((*::std::mem::transmute::<&[u8; 22],
                                                &mut [libc::c_char; 22]>(b"5 == buffer_size(buf)\x00")).as_mut_ptr(),
                      (*::std::mem::transmute::<&[u8; 7],
                                                &mut [libc::c_char; 7]>(b"test.c\x00")).as_mut_ptr(),
                      228i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"void test_buffer_compact()\x00")).as_ptr());
    };
    equal((*::std::mem::transmute::<&[u8; 6],
                                    &mut [libc::c_char; 6]>(b"Hello\x00")).as_mut_ptr(),
          (*buf).data);
    buffer_free(buf);
}
unsafe fn main_0() -> libc::c_int {
    test_buffer_new();
    test_buffer_new_with_size();
    test_buffer_append();
    test_buffer_append__grow();
    test_buffer_append_n();
    test_buffer_prepend();
    test_buffer_slice();
    test_buffer_slice__range_error();
    test_buffer_slice__end();
    test_buffer_slice__end_overflow();
    test_buffer_equals();
    test_buffer_indexof();
    test_buffer_fill();
    test_buffer_clear();
    test_buffer_trim();
    test_buffer_compact();
    printf((*::std::mem::transmute::<&[u8; 26],
                                     &mut [libc::c_char; 26]>(b"\n  \x1b[32m\xe2\x9c\x93 \x1b[90mok\x1b[0m\n\n\x00")).as_mut_ptr());
    return 0i32;
}
pub fn main() -> () { unsafe { ::std::process::exit(main_0() as i32) } }
