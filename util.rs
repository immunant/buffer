use libc::*;
use std::cmp::min;

pub fn strlen(str: &[c_char]) -> size_t {
    str
        .iter()
        .position(|&c| c == 0)
        .expect("input string isn't null terminated") as size_t
}

pub fn strncat(dest: &mut [c_char], src: &[c_char], num: size_t) {
    let dlen = strlen(dest);
    let slen = strlen(src);
    // If the length of the C string in source is less than num,
    // only the content up to the terminating null-character is copied.
    let num = min(num, slen);
    let needed = dlen + num + 1;
    if dest.len() < needed {
        panic!("Pointer to the destination slice should be large enough \
                to contain the concatenated resulting string, including the \
                additional null-character.");
    }
    let ds = &mut dest[dlen..dlen + num];
    let ss = &src[..num];
    ds.copy_from_slice(ss);
    dest[needed - 1] = 0; // null terminator
}

pub fn isspace(c: c_int) -> c_int {
    match char::from_u32(c as u32).unwrap().is_ascii_whitespace() {
        true => 1 as c_int,
        false => 0 as c_int
    }
}

#[macro_export]
macro_rules! c_slice {
    ($str:expr) => {{
        let ptr = ::byte_strings::c_str!($str).as_ptr();
        unsafe { std::slice::from_raw_parts(ptr, $str.len() + 1) }
    }};
}