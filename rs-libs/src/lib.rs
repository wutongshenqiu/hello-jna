#[repr(C)]
#[derive(Debug)]
pub struct SimpleStruct {
    v: u32,
}

#[no_mangle]
pub unsafe extern "C" fn construct_ss_ref(v: u32) -> *mut SimpleStruct {
    Box::into_raw(Box::new(SimpleStruct { v }))
}

#[no_mangle]
pub unsafe extern "C" fn construct_ss_val(v: u32) -> SimpleStruct {
    SimpleStruct { v }
}

#[no_mangle]
pub unsafe extern "C" fn getv_by_ref(ss: *const SimpleStruct) -> u32 {
    (*ss).v
}

#[no_mangle]
pub unsafe extern "C" fn getv_by_val(ss: SimpleStruct) -> u32 {
    ss.v
}

#[no_mangle]
pub unsafe extern "C" fn sum_by_refs(ss: *const *const SimpleStruct, len: usize) -> u32 {
    let mut res = 0;
    for i in 0..len {
        res += (*(*ss.add(i))).v;
    }

    res
}

#[no_mangle]
pub unsafe extern "C" fn sum_by_vals(ss: *const SimpleStruct, len: usize) -> u32 {
    let mut res = 0;
    for i in 0..len {
        res += (*ss.add(i)).v;
    }

    res
}
