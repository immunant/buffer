use libc::*;

pub fn strlen(str: &[c_char]) -> size_t {
    str
        .iter()
        .position(|&c| c == 0)
        .expect("input string isn't null terminated") as size_t
}

pub fn strncat(mut dest: &[c_char], src: &[c_char], count: size_t) {
    let mut t = dest.to_vec();
    t.append(&mut src[..count].to_vec());
    dest = &t[..];
}

#[macro_export]
macro_rules! c_slice {
    ($str:expr) => {{
        let ptr = ::byte_strings::c_str!($str).as_ptr();
        unsafe { std::slice::from_raw_parts(ptr, $str.len() + 1) }
    }};
}