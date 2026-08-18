#[no_mangle]
#[repr(C, packed)]
pub unsafe extern "C" fn memset(ptr: *mut u8, c: i32, i: usize) -> *mut u8 {
    let byte = c as u8;
    let mut idx: usize = 0;

    while idx < i {
        *ptr.add(idx) = byte;
        idx += 1;
    }
    ptr
}
