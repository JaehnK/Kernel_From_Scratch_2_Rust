#[no_mangle]
#[repr(C, packed)]
#[allow(dead_code)]
pub unsafe extern "C" fn memset(ptr: *mut u8, c: u8, i: usize) -> *mut u8 {
    let mut idx: usize = 0;
    let start_ptr = ptr;

    while (idx < i) {
        *ptr = c;
        idx += 1;
        ptr = ptr.add(1);
    }
    start_ptr
}
