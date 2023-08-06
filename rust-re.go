package go_rust_regex

/*
#cgo LDFLAGS: ./lib/rust-re/target/release/librust_re.a -ldl
#include "./lib/rust-re.h"
#include "stdlib.h"
*/
import "C"

import (
	"errors"
	"runtime"
	"strings"
	"unsafe"
)

//go:generate bash -c "cargo clean --manifest-path=./lib/rust-re/Cargo.toml && cargo build --release --manifest-path=./lib/rust-re/Cargo.toml"

type RustRe struct {
	ptr unsafe.Pointer
}

func rustReCompile(pattern string) (unsafe.Pointer, error) {
	pat := C.CString(pattern)
	ptr := C.compile(pat)

	C.free(unsafe.Pointer(pat))

	if ptr == nil {
		return nil, errors.New("Could not compile regex")
	}

	return ptr, nil
}

func rustReDestroy(ptr unsafe.Pointer) bool {
	return bool(C.destroy(ptr))
}

func rustReReplace(ptr unsafe.Pointer, text string, replacement string) string {

	txt := C.CString(text)
	rep := C.CString(replacement)

	res := C.replace(ptr, txt, rep)

	C.free(unsafe.Pointer(txt))
	C.free(unsafe.Pointer(rep))

	ret := strings.Clone(C.GoString(res))

	C.destroy_cstr(res)
	return ret
}

func RustReMustCompile(pattern string) *RustRe {

	ptr, err := rustReCompile(pattern)

	if err != nil {
		panic(err)
	}

	re := &RustRe{
		ptr: ptr,
	}

	runtime.SetFinalizer(re, func(f *RustRe) {
		rustReDestroy(f.ptr)
	})

	return re
}

func (re *RustRe) Replace(text string, replacement string) string {
	return rustReReplace(re.ptr, text, replacement)
}
