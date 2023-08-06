#![feature(vec_into_raw_parts)]
use regex::Regex;
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
};

/// # Safety
///
/// todo
#[no_mangle]
pub extern "C" fn compile(name: *const libc::c_char) -> *const libc::c_void {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name = name_cstr.to_str();

    if name.is_err() {
        return std::ptr::null();
    }

    let re = Regex::new(name.unwrap());

    if re.is_err() {
        return std::ptr::null();
    }

    let re = re.unwrap();

    let boxed_re = Box::new(re);

    Box::into_raw(boxed_re) as *mut libc::c_void
}

/// # Safety
///
/// todo
#[no_mangle]
pub unsafe extern "C" fn compile_bytes(name: *const libc::c_char) -> *const libc::c_void {
    let name_cstr = CStr::from_ptr(name) ;
    let name = name_cstr.to_str();

    if name.is_err() {
        return std::ptr::null();
    }

    let re = regex::bytes::Regex::new(name.unwrap());

    if re.is_err() {
        return std::ptr::null();
    }

    let re = re.unwrap();

    let boxed_re = Box::new(re);

    Box::into_raw(boxed_re) as *mut libc::c_void
}

/// # Safety
///
/// todo
#[no_mangle]
pub unsafe extern "C" fn destroy(re: *mut libc::c_void) -> bool {
    if re.is_null() {
        return false;
    }

    let _ = Box::from_raw(re as *mut Regex);

    true
}

/// # Safety
///
/// todo
#[no_mangle]
pub unsafe extern "C" fn destroy_bytes(re: *mut libc::c_void) -> bool {
    if re.is_null() {
        return false;
    }

    let _ = Box::from_raw(re as *mut regex::bytes::Regex);

    true
}

/// # Safety
///
/// todo
#[no_mangle]
pub unsafe extern "C" fn destroy_cstr(string: *mut libc::c_char) -> bool {
    if string.is_null() {
        return false;
    }
   
    _ = CString::from_raw(string);
    

    true
}

/// # Safety
///
/// todo
#[no_mangle]
pub unsafe extern "C" fn replace(
    re: *mut libc::c_void,
    src: *const libc::c_char,
    rep: *const libc::c_char,
) -> *const libc::c_char {
    if re.is_null() {
        return std::ptr::null();
    }

    let src_cstr = CStr::from_ptr(src);
    let src = src_cstr.to_str();

    if src.is_err() {
        return std::ptr::null();
    }

    let rep_cstr = CStr::from_ptr(rep) ;
    let rep = rep_cstr.to_str();

    if rep.is_err() {
        return std::ptr::null();
    }

    let src = src.unwrap();
    let rep = rep.unwrap();

    let re = re as *mut Regex;

    let result = (*re).replace_all(src, rep);

    let cstr = match result {
        Cow::Borrowed(s) => CString::new(s),
        Cow::Owned(s) => CString::new(s),
    };

    if cstr.is_err() {
        return std::ptr::null();
    }

    let cstr = cstr.unwrap();

    cstr.into_raw() as *const libc::c_char
}

/// # Safety
///
/// todo
#[no_mangle]
pub unsafe extern "C" fn replace_bytes(
    re: *mut libc::c_void,
    src: *const libc::c_char,
    src_len: usize,
    rep: *const libc::c_char,
    rep_len: usize,
    result_len: *mut usize,
) -> *const libc::c_char {
    if re.is_null() {
        return std::ptr::null();
    }

    let src_bytes = std::ptr::slice_from_raw_parts(src as *const u8, src_len);

    let rep_bytes = std::ptr::slice_from_raw_parts(rep as *const u8, rep_len);

    let re = re as *mut regex::bytes::Regex;

    let result = unsafe { (*re).replace_all(&*src_bytes, &*rep_bytes) };

    match result {
        Cow::Borrowed(s) => {
            *result_len = s.len();
            s as *const [u8] as *const libc::c_char
        }
        Cow::Owned(s) => {
            *result_len = s.len();
            let (ptr, _, _) = s.into_raw_parts();
            ptr as *const libc::c_char
        }
    }
}
