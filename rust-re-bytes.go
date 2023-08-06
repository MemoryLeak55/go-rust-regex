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
	"unsafe"
)

type RustReBytes struct {
	ptr unsafe.Pointer
}

func rustReCompileBytes(pattern string) (unsafe.Pointer, error) {
	pat := C.CString(pattern)
	ptr := C.compile_bytes(pat)

	C.free(unsafe.Pointer(pat))

	if ptr == nil {
		return nil, errors.New("Could not compile regex")
	}

	return ptr, nil
}

func rustReDestroyBytes(ptr unsafe.Pointer) bool {
	return bool(C.destroy_bytes(ptr))
}

func rustReReplaceBytes(ptr unsafe.Pointer, text []byte, replacement []byte) []byte {

	textPtr := C.CBytes(text)
	repPtr := C.CBytes(replacement)
	outSize := C.long(0)

	res := C.replace_bytes(ptr, textPtr, C.long(len(text)), repPtr, C.long(len(replacement)), &outSize)

	if unsafe.Pointer(res) == textPtr {
		C.free(repPtr)

		return text
	}

	C.free(textPtr)
	C.free(repPtr)
	data := C.GoBytes(unsafe.Pointer(res), C.int(outSize))
	return data
}

func RustReMustCompileBytes(pattern string) *RustReBytes {

	ptr, err := rustReCompileBytes(pattern)

	if err != nil {
		panic(err)
	}

	re := &RustReBytes{
		ptr: ptr,
	}

	runtime.SetFinalizer(re, func(f *RustReBytes) {
		rustReDestroyBytes(f.ptr)
	})

	return re
}

func (re *RustReBytes) Replace(text []byte, replacement []byte) []byte {
	return rustReReplaceBytes(re.ptr, text, replacement)
}
