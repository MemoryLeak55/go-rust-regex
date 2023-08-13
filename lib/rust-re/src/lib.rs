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
    let ptr = Box::into_raw(boxed_re);
    ptr as *const libc::c_void
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
    re: *const libc::c_void,
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

    let rep_cstr = CStr::from_ptr(rep);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace_all() {
        unsafe {
            let regex = CString::new(".").unwrap().into_raw();
            let re = compile(regex as *const _);
            let rep = CString::new("b").unwrap().into_raw();
            let src = CString::new("cat").unwrap().into_raw();
            let result = replace(re, src, rep);

            let res = CString::from_raw(result as *mut _);

            let res_str = res.to_str().unwrap();

            assert_eq!(res_str, "bbb");

            destroy(re as *mut _);
            destroy_cstr(rep as *mut _);
            destroy_cstr(src as *mut _);
            destroy_cstr(regex as *mut _);
        }
    }
}
